pub mod grpc {
    tonic::include_proto!("echo.v1");
    tonic::include_proto!("dictionary.v1");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("file_descriptor");
}

pub mod auth;
pub mod db_client;
pub mod services;
