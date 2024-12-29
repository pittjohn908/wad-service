
pub mod grpc {
    tonic::include_proto!("echo");
    tonic::include_proto!("dictionary");
}

pub mod services;
pub mod db_client;