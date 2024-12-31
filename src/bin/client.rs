use tonic::Request;
use wad_service::grpc::dictionary_service_client::DictionaryServiceClient;
use wad_service::grpc::echo_service_client::EchoServiceClient;
use wad_service::grpc::{EchoRequest, GetRandomWordRequest};


#[tokio::main]
async fn main() -> anyhow::Result<()>  {
    
    let mut client = EchoServiceClient::connect("http://localhost:50051").await?;
    let request = Request::new(EchoRequest { message: "Testing Echo Endpoint".into() });
    let response = client.echo(request).await?;
    println!("RESPONSE={:?}", response);

    let mut client = DictionaryServiceClient::connect("http://localhost:50051").await?;
    let dictionary_request = Request::new(GetRandomWordRequest {});
    let response = client.get_random_word(dictionary_request).await?;
    println!("RESPONSE={:?}", response);


    Ok(())
}