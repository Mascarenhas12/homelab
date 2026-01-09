use clap::{Parser, Subcommand};
use log::{info, warn};
use manulab::resource_provider_client::ResourceProviderClient;


pub mod manulab {
    tonic::include_proto!("manulab");
}

#[tokio::main]
async fn get_plugin_info() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = match ResourceProviderClient::connect("http://[::1]:50051").await {
        Ok(client) => client,
        Err(error) => {
          warn!("Problem connecting to server. Make sure server is up and running.");
          panic!("Closing client: {error:?}");
        }
    };
    info!("Connected to server!");

    let request = tonic::Request::new(());
    let response = client.get_plugin_info(request).await?;

    info!("RESPONSE={response:?}");

    Ok(())
}

/*
#[derive(Debug, Clone)]
enum ManifestFormat {
    Text,
    Json,
    Yaml,
    Table,
}

#[derive(Debug, Subcommand, Clone)]
enum ShipYardCommand {
    Install{
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value_t = true)]
        local: bool,
        #[arg(short, long, default_value_t = "latest")]
        version: String,
    },
    Manifest{
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value_t = ManifestFormat::Text)]
        format: ManifestFormat,
    },
    Remove{
        #[arg(short, long)]
        name: String,
    },
}
*/

#[derive(Debug, Subcommand)]
enum CaravelaCommand {
    Shipyard,
    Sail,
    Wreck,
    Spot,
}

#[derive(Parser, Debug)]
#[command(name = "caravela", version, about, long_about = "Rust client to interact with Manulab GRPC server")]
pub struct Cli {
    #[clap(subcommand)]
    command: CaravelaCommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
  env_logger::init();
  let cli = Cli::parse();
  match &cli.command {
      CaravelaCommand::Shipyard => {
          info!("Received command shipyard!");
      },
      CaravelaCommand::Sail => {
          info!("Received command sail!");
      },
      CaravelaCommand::Wreck => {
          info!("Received command wreck");
      },
      CaravelaCommand::Spot => {
          info!("Received command spot");
      },
  }
  Ok(())
}
