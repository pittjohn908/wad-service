use wad_service::echo_client::EchoClient;
use wad_service::EchoRequest;


#[tokio::main]
async fn main() -> anyhow::Result<()>  {
    let mut client = EchoClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(EchoRequest { message: "Testing Echo Endpoint".into() });

    let response = client.echo(request).await?;

        println!("RESPONSE={:?}", response);


    Ok(())
}