use tonic::async_trait;
use tonic::transport::Server;
use wad_service::echo_server::{Echo, EchoServer};
use wad_service::{EchoReply, EchoRequest};

pub struct MyEcho;

#[async_trait]
impl Echo for MyEcho {
    async fn echo(
        &self,
        request: tonic::Request<EchoRequest>,
    ) -> Result<tonic::Response<EchoReply>, tonic::Status> {
        Ok(tonic::Response::new(EchoReply {
            message: format!("Echoing back: {}", request.get_ref().message)
        }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse().unwrap();

    Server::builder()
        .add_service(EchoServer::new(MyEcho))
        .serve(addr)
        .await?;

    Ok(())
}