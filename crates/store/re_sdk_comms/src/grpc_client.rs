#![allow(clippy::disallowed_methods)]
use std::{net::SocketAddr, thread, time::Duration};

use crossbeam::channel::{Receiver, Sender};
use re_log_types::{LogMsg, StoreKind};
use re_smart_channel::TryRecvError;
use re_storagenode_types::{
    external::re_chunk_store::ChunkStoreEvent,
    storage::{
        storage_node_client::StorageNodeClient, DataStoreId, DataStoreKind, InsertData,
        SubscribeRequest,
    },
};
use tokio::time::sleep;
use tokio_stream::StreamExt;
use url::Url;

#[derive(Debug)]
pub struct GrpcClient {
    msg_tx: Sender<LogMsg>,
}

impl GrpcClient {
    // currently used by the LogSink (GrpcSink) for sending updates to the storage node
    pub fn new(addr: SocketAddr) -> Self {
        let (msg_tx, msg_rx) = crossbeam::channel::unbounded();

        std::thread::spawn(move || {
            insert_stream(addr, msg_rx);
        });

        // here we're both creating a sender that grpc log sink can use to send data to SN
        // and a receiver that can be used to receive updates from SN
        Self { msg_tx }
    }

    // used for viewer to subscribe to remote store updates and react to that accordingly
    pub fn subscribe_to_updates(addr: SocketAddr) -> Receiver<Vec<ChunkStoreEvent>> {
        let (event_tx, event_rx) = crossbeam::channel::unbounded();

        let url = Url::parse(&format!("http://{}:{}", addr.ip(), addr.port()))
            .expect("failed to parse grpc url");

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        thread::spawn(move || {
            let mut updates_stream = rt.block_on(async {
                let mut client = StorageNodeClient::connect(url.to_string())
                    .await
                    .expect("failed to connect to grpc server");

                client
                    .subscribe_to_updates(SubscribeRequest { subscriber_id: 1 })
                    .await
                    .unwrap()
                    .into_inner()
            });

            rt.block_on(async move {
                while let Some(update) = updates_stream.next().await {
                    match update {
                        Ok(update) => {
                            // this whole conversion of chunk store event to data store event is pretty silly...
                            let cs_update_events = update
                                .store_events
                                .into_iter()
                                .map(|ev| ev.try_into().unwrap())
                                .collect::<Vec<ChunkStoreEvent>>();
                            re_log::info!(
                                "received update from the server, {} events",
                                cs_update_events.len()
                            );

                            event_tx
                                .send(cs_update_events)
                                .expect("failed to send update to the channel");
                        }
                        Err(e) => {
                            re_log::error!("server replied with an error: {:?}", e);
                            break;
                        }
                    }
                }
            });
        });

        event_rx
    }

    pub fn send(&self, log_msg: LogMsg) {
        self.msg_tx.send(log_msg).ok();
    }
}
fn insert_stream(addr: SocketAddr, msg_rx: Receiver<LogMsg>) {
    let url = Url::parse(&format!("http://{}:{}", addr.ip(), addr.port()))
        .expect("failed to parse grpc url");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut client = rt.block_on(async {
        StorageNodeClient::connect(url.to_string())
            .await
            .expect("failed to connect to grpc server")
    });

    // stream of insert requests
    let chunks_stream = async_stream::stream! {
        loop {
            match msg_rx.try_recv() {
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {
                    sleep(Duration::from_millis(10)).await;
                },
                Ok(log_msg) => {
                    if let LogMsg::ArrowMsg(store_id, log_msg) = log_msg {
                        let encoding_options = re_log_encoding::EncodingOptions::UNCOMPRESSED;
                        let encoded_msg = re_log_encoding::encoder::encode_to_bytes(re_build_info::CrateVersion::LOCAL, encoding_options, vec![&LogMsg::ArrowMsg(store_id.clone(), log_msg)]).unwrap();
                        let insert_request = InsertData {
                            store_id: Some(DataStoreId {
                                id: (*store_id.id).clone(),
                                kind: match store_id.kind {
                                    StoreKind::Blueprint => DataStoreKind::Blueprint.try_into().unwrap(),
                                    StoreKind::Recording => DataStoreKind::Recording.try_into().unwrap(),
                                },
                            }),
                            chunk_data: encoded_msg,
                        };

                        re_log::debug!("received and encoded next log msg");
                        yield insert_request;
                    }
                }
            }
        }
    };

    rt.block_on(async move {
        let mut insert_response_stream = client
            .insert(chunks_stream)
            .await
            .expect("failed to send message to the server")
            .into_inner();

        while let Some(resp) = insert_response_stream.next().await {
            match resp {
                Ok(resp) => {
                    for ev in resp.store_event {
                        re_log::debug!(
                            "server replied successfully with an event {:?}",
                            ev.event_id
                        );
                    }
                }
                Err(e) => {
                    re_log::error!("server replied with an error: {:?}", e);
                    break;
                }
            }
        }
    });
}
