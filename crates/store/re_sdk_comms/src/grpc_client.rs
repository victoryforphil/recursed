use std::time::Duration;

use crossbeam::channel::{Receiver, Sender};
use data::{data_ingestion_client::DataIngestionClient, RawLogMsg};
use re_log_types::LogMsg;
use re_smart_channel::TryRecvError;
use tokio::{sync::mpsc, time::sleep};
use tokio_stream::StreamExt;

pub mod data {
    tonic::include_proto!("ingestion");
}

#[derive(Debug)]
pub struct Client {
    msg_tx: Sender<LogMsg>,
}

#[allow(clippy::new_without_default)]
impl Client {
    pub fn new() -> Self {
        let (msg_tx, msg_rx) = crossbeam::channel::unbounded();
        let (async_tx, async_rx) = mpsc::channel(32);

        tokio::spawn(async move {
            pass_messages(msg_rx, async_tx).await;
        });

        tokio::spawn(async move {
            grpc_sender(async_rx).await;
        });

        Self { msg_tx }
    }

    pub fn send(&self, log_msg: LogMsg) {
        self.msg_tx.send(log_msg).ok();
    }
}

async fn grpc_sender(mut msg_rx: tokio::sync::mpsc::Receiver<LogMsg>) {
    let mut client = DataIngestionClient::connect("http://127.0.0.1:50051")
        .await
        .expect("failed to connect to grpc server");

    let encoding_options = re_log_encoding::EncodingOptions::UNCOMPRESSED;

    let chunks_stream = async_stream::stream! {
        loop {
            if let Some(log_msg) = msg_rx.recv().await {
                let encoded_msg = re_log_encoding::encoder::encode_to_bytes(re_build_info::CrateVersion::LOCAL, encoding_options, std::iter::once(&log_msg)).expect("failed to encode LogMsg");
                let raw_log_msg = RawLogMsg { data: encoded_msg};
                re_log::debug!("received and encoded next log msg");
                yield raw_log_msg;
            } else {
                re_log::debug!("upstream channel closed, stopping grpc stream");
                break;
            }
        }
    };

    let mut response_stream = client
        .record(chunks_stream)
        .await
        .expect("failed to send message to the server")
        .into_inner();

    while let Some(re) = response_stream.next().await {
        match re {
            Ok(re_good) => {
                re_log::trace!("sent a message successfully: {}", re_good.status);
            }
            Err(e) => {
                // should bail out?
                re_log::error!("server replied with an error: {:?}", e);
            }
        }
    }
}

// this is where we bridge our blocking code with async world
async fn pass_messages(
    msg_rx_sync: Receiver<LogMsg>,
    msg_tx_async: tokio::sync::mpsc::Sender<LogMsg>,
) {
    loop {
        // we use non-blocking call intentionally as this whole function runs in async context i.e.
        // tokio is responsible for scheduling, and despite the fact we might be running with >1 worker
        // threads, we might block / starve out other tasks, including the grpc sender task
        match msg_rx_sync.try_recv() {
            Ok(log_msg) => {
                msg_tx_async
                    .send(log_msg)
                    .await
                    .expect("failed to send to async channel");
            }
            Err(TryRecvError::Empty) => {
                sleep(Duration::from_millis(10)).await;
            }
            Err(TryRecvError::Disconnected) => {
                re_log::info!("sender disconnected, stopping message passing");
                break;
            }
        }
    }
}
