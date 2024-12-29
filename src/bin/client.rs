use tonic::Request;
use wad_service::grpc::dictionary_client::DictionaryClient;
use wad_service::grpc::echo_client::EchoClient;
use wad_service::grpc::{EchoRequest, GetRandomWordRequest};


#[tokio::main]
async fn main() -> anyhow::Result<()>  {
    let mut client = EchoClient::connect("http://[::1]:50051").await?;
    let request = Request::new(EchoRequest { message: "Testing Echo Endpoint".into() });
    let response = client.echo(request).await?;
    println!("RESPONSE={:?}", response);

    let mut client = DictionaryClient::connect("http://[::1]:50051").await?;
    let dictionary_request = Request::new(GetRandomWordRequest {});
    let response = client.get_random_word(dictionary_request).await?;
    println!("RESPONSE={:?}", response);


    Ok(())
}