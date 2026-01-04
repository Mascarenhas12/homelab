use std::env;
use tonic::{transport::Server, Request, Response, Status};

use opentelemetry::{global, KeyValue};
use opentelemetry::metrics::{Meter, Counter};
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::Resource;

use tracing::{instrument, event, Level};
use tracing_subscriber::fmt;

use manulab_pulumi::resource_provider_server::{ResourceProvider, ResourceProviderServer};
use manulab_pulumi::{PluginInfo, GetSchemaRequest, GetSchemaResponse};

fn init_meter_provider() -> opentelemetry_sdk::metrics::SdkMeterProvider {
    let exporter = opentelemetry_stdout::MetricExporterBuilder::default().build();
    let provider = SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(
            Resource::builder()
                .with_service_name("manulab-grpc-server")
                .build(),
        )
        .build();
    global::set_meter_provider(provider.clone());
    provider
}

pub mod manulab_pulumi {
    tonic::include_proto!("manulab");
}

// This is a rust macro 
#[derive(Debug)]
pub struct ManulabProvider {
    meter: Meter,
    counters: Vec<Counter<u64>>,
}

impl ManulabProvider {
    pub fn new() -> ManulabProvider {
        let meter = global::meter("get-plugin-info");
        let counters = Vec::with_capacity(1);

        let format = fmt::format()
            .with_level(true)
            .with_target(false)
            .with_thread_ids(true)
            .with_thread_names(true)
            .pretty();

        let sub = tracing_subscriber::fmt()
            .event_format(format)
            .init();

        ManulabProvider{
            meter: meter,
            counters: counters,
        }
    }

    pub fn add_counter(&mut self, count: Counter<u64>) {
        self.counters.push(count);
    }
}


#[tonic::async_trait]
impl ResourceProvider for ManulabProvider {
    #[instrument]
    async fn get_plugin_info(
        &self,
        request: Request<()>, // Accept empty request
        ) -> Result<Response<PluginInfo>, Status> { // Return instance of PluginInfo message

            self.counters[0].add(
                1,
                &[
                    KeyValue::new("plugin_version","rc0-v0.0.1"),
                ]
            );
            event!(Level::INFO, "Got a request for PluginInfo from {:#?}", request.remote_addr());

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
    
    let meter_provider = init_meter_provider();
    let mut manulab = ManulabProvider::new();
    let count = manulab.meter.u64_counter("get-plugin-info-count").build();
    manulab.add_counter(count);

    event!(Level::DEBUG, "Running on port: {}", port);
    Server::builder()
        .add_service(ResourceProviderServer::new(manulab))
        .serve(addr)
        .await?;

    meter_provider.shutdown()?;
    Ok(())
}
