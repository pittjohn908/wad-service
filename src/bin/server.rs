use sqlx::PgPool;
use tonic::async_trait;
use tonic::transport::Server;
use wad_service::grpc::dictionary_service_server::DictionaryServiceServer;
use wad_service::grpc::echo_service_server::{EchoService, EchoServiceServer};
use wad_service::grpc::{EchoResponse, EchoRequest};
use wad_service::services::dictionary::DictionaryGrpcService;

pub struct MyEcho;

#[async_trait]
impl EchoService for MyEcho {
    async fn echo(
        &self,
        request: tonic::Request<EchoRequest>,
    ) -> Result<tonic::Response<EchoResponse>, tonic::Status> {
        Ok(tonic::Response::new(EchoResponse {
            message: format!("Echoing back: {}", request.get_ref().message)
        }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:50051".parse().unwrap();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(wad_service::grpc::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;
    let dictionary_service = DictionaryGrpcService::from(pool.clone());

    Server::builder()
        .add_service(reflection_service)
        .add_service(EchoServiceServer::new(MyEcho))
        .add_service(DictionaryServiceServer::new(dictionary_service))
        .serve(addr)
        .await?;

    Ok(())
}