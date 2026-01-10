use clap::{Args, Parser, Subcommand};
use log::{info, warn, error};
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
*/

#[derive(Debug, Subcommand)]
enum ShipyardSubCommand {
    Install{
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value_t = true)]
        local: bool,
        #[arg(short, long)]
        version: String,
    },
    Manifest{
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        format: String,
    },
    Remove{
        #[arg(short, long)]
        name: String,
    },
}


#[derive(Args, Debug)]
struct ShipyardCommands {
    #[clap(subcommand)]
    command: ShipyardSubCommand,
} 

#[derive(Debug, Subcommand)]
enum CaravelaCommand {
    Shipyard(ShipyardCommands),
    Sail,
    Wreck,
    Spot,
}

#[derive(Parser, Debug)]
#[command(propagate_version = true)]
#[command(name = "caravela", version, about, long_about = "Rust client to interact with Manulab GRPC server")]
pub struct Cli {
    #[clap(subcommand)]
    command: CaravelaCommand,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
  env_logger::init();
  let cli = Cli::parse();
  match &cli.command {
      CaravelaCommand::Shipyard(sub) => {
          info!("Received command shipyard!");
          match &sub.command {
              ShipyardSubCommand::Install {..} => info!("Received subcommand Install"),
              ShipyardSubCommand::Manifest {..} => return get_plugin_info(),
              ShipyardSubCommand::Remove {..} => error!("Cannot remove only shipyard"),
          }
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
