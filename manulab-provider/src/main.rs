use std::env;
use tonic::{transport::Server, Request, Response, Status};

use manulab_pulumi::homelab_provider_server::{HomelabProvider, HomelabProviderServer};
use manulab_pulumi::{PluginInfo, GetSchemaRequest, GetSchemaResponse};

pub mod manulab_pulumi {
    tonic::include_proto!("manulab");
}

// TODO: findout what this syntax is
#[derive(Debug, Default)]
pub struct ManulabProvider {}

#[tonic::async_trait]
impl HomelabProvider for ManulabProvider {
    async fn get_plugin_info(
        &self,
        request: Request<()>, // Accept empty request
        ) -> Result<Response<PluginInfo>, Status> { // Return instance of PluginInfo message
            println!("Got a request for PluginInfo from: {:#?}", request.remote_addr());

            let reply = PluginInfo {
                version: "rc0-v0.0.1".to_string(),
            };

            Ok(Response::new(reply))
    }

    async fn get_schema(
        &self,
        request: Request<GetSchemaRequest>,
        ) -> Result<Response<GetSchemaResponse>, Status> {
        unimplemented!()
    }

    async fn configure(
        &self,
        request: Request<()>,
        ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn check(
        &self,
        request: Request<()>,
        ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn diff(
        &self,
        request: Request<()>,
        ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn create(
        &self,
        request: Request<()>,
        ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn read(
        &self,
        request: Request<()>,
        ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn update(
        &self,
        request: Request<()>,
        ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn delete(
        &self,
        request: Request<()>,
        ) -> Result<Response<()>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or("50051".to_string());
    // TODO: error handle as logs
    let addr = format!("[::1]:{}", port).parse()?;
    
    let manulab = ManulabProvider::default();

    println!("Running on port {}...", port);
    Server::builder()
        .add_service(HomelabProviderServer::new(manulab))
        .serve(addr)
        .await?;

    Ok(())
}
