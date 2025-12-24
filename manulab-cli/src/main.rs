use manulab::resource_provider_client::ResourceProviderClient;

pub mod manulab {
    tonic::include_proto!("manulab");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ResourceProviderClient::connect("http://[::1]:50051").await?;
    println!("Conencted to Server!");

    let request = tonic::Request::new(());
    let response = client.get_plugin_info(request).await?;

    println!("RESPONSE={response:?}");

    Ok(())
}
