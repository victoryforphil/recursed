use data::{
    data_ingestion_server::{DataIngestion, DataIngestionServer},
    RawLogMsg, RecordingResponse,
};
use re_log_types::LogMsg;
use re_smart_channel::{Receiver, Sender};
use std::{net::ToSocketAddrs, pin::Pin};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{transport::Server, Response, Status};

pub mod data {
    tonic::include_proto!("ingestion");
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<RecordingResponse, Status>> + Send>>;

pub struct IngestionServer {
    tx: Sender<LogMsg>,
}

#[tonic::async_trait]
impl DataIngestion for IngestionServer {
    type RecordStream = ResponseStream;

    async fn record(
        &self,
        request: tonic::Request<tonic::Streaming<RawLogMsg>>,
    ) -> std::result::Result<tonic::Response<Self::RecordStream>, tonic::Status> {
        let mut msg_stream = request.into_inner();
        let (grpc_tx, grpc_rx) = mpsc::channel(128);
        let tx_clone = self
            .tx
            .clone_as(re_smart_channel::SmartMessageSource::TcpClient { addr: None });

        tokio::spawn(async move {
            while let Some(raw_msg) = msg_stream.next().await {
                re_log::trace!("received new message over grpc");
                match raw_msg {
                    Ok(m) => {
                        let raw_bytes = m.data;
                        let version_policy = re_log_encoding::decoder::VersionPolicy::Warn;
                        for msg in
                            re_log_encoding::decoder::decode_bytes(version_policy, &raw_bytes)
                                .expect("couldn't decode message from grpc client, exploding")
                        {
                            if let Err(e) = tx_clone.send(msg) {
                                re_log::error!("failed to send msg upstream, error: {:?}", e);
                            }

                            // notify client we've successfully passed this message upstream
                            grpc_tx
                                .send(Ok(RecordingResponse { status: 200 }))
                                .await
                                .expect("failed to send grpc reply to client");
                        }
                    }
                    Err(e) => {
                        re_log::error!("failed to receive message from the client: {}", e);
                        break;
                    }
                }
            }
        });

        let out_stream = ReceiverStream::new(grpc_rx);

        Ok(Response::new(Box::pin(out_stream) as ResponseStream))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GrpcServerError {
    #[error("Failed to start gRPC server. Another Rerun instance is probably running. {err}")]
    BindError { err: tonic::transport::Error },
}

pub async fn serve() -> Result<Receiver<LogMsg>, GrpcServerError> {
    let (tx, rx) = re_smart_channel::smart_channel(
        re_smart_channel::SmartMessageSource::Unknown,
        re_smart_channel::SmartChannelSource::GrpcServer,
    );

    let ingestion = IngestionServer { tx };

    tokio::spawn(async move {
        re_log::info!("starting grpc server for the viewer...");
        Server::builder()
            .add_service(DataIngestionServer::new(ingestion))
            // hardcoded address
            .serve("0.0.0.0:50051".to_socket_addrs().unwrap().next().unwrap())
            .await
            .map_err(|err| GrpcServerError::BindError { err })
            .expect("failed to start grpc server");
    });

    Ok(rx)
}
