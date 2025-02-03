use tonic::Request;
use wad_service::grpc::{dictionary_service_client::DictionaryServiceClient, GetRandomWordRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = DictionaryServiceClient::connect("http://localhost:50051").await?;
    let dictionary_request = Request::new(GetRandomWordRequest {});
    let response = client.get_random_word(dictionary_request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
