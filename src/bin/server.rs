use std::sync::Arc;

use sqlx::PgPool;
use tonic::async_trait;
use tonic::transport::Server;
use tonic_async_interceptor::async_interceptor;
use wad_service::auth::apple_keys::AppleKeyManager;
use wad_service::auth::auth_utils::check_auth;
use wad_service::grpc::dictionary_service_server::DictionaryServiceServer;
use wad_service::grpc::echo_service_server::{EchoService, EchoServiceServer};
use wad_service::grpc::{EchoRequest, EchoResponse};
use wad_service::services::dictionary_service::DictionaryGrpcService;

pub struct MyEcho;

#[async_trait]
impl EchoService for MyEcho {
    async fn echo(
        &self,
        request: tonic::Request<EchoRequest>,
    ) -> Result<tonic::Response<EchoResponse>, tonic::Status> {
        Ok(tonic::Response::new(EchoResponse {
            message: format!("Echoing back: {}", request.get_ref().message),
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

    let apple_key_manager = Arc::new(AppleKeyManager::new());
    if let Err(err) = apple_key_manager.refresh_keys().await {
        println!("Error refreshing keys: {:?}", err);
    }

    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    let auth_interceptor = move |req| {
        let apple_key_manager = apple_key_manager.clone();
        async move { check_auth(req, apple_key_manager).await }
    };

    Server::builder()
        .layer(async_interceptor(auth_interceptor))
        .add_service(reflection_service)
        .add_service(EchoServiceServer::new(MyEcho))
        .add_service(DictionaryServiceServer::new(DictionaryGrpcService::from(
            pool.clone(),
        )))
        .serve(addr)
        .await?;

    Ok(())
}
