//use clap::Parser;
use log::{info, warn};
use manulab::resource_provider_client::ResourceProviderClient;

pub mod manulab {
    tonic::include_proto!("manulab");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut client = match ResourceProviderClient::connect("http://[::1]:50051").await {
        Ok(client) => client,
        Err(error) => {
          warn!("Problem connecting to server.Make sure server is up and running.");
          panic!("Closing client: {error:?}");
        }
    };
    info!("Connected to server!");

    let request = tonic::Request::new(());
    let response = client.get_plugin_info(request).await?;

    println!("RESPONSE={response:?}");

    Ok(())
}

/*#[derive(Parser)]
struct Cli {
    command: Command;
    args: Args;
}

fn main() {
  let 
}*/
