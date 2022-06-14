mod service;
mod log_layer;

pub mod model {
    tonic::include_proto!("idgen");
}

use std::sync::Arc;
use std::time::Instant;
use tonic::transport::Server;
use model::id_gen_server::IdGenServer;

use snowflake::structure::IdStructure;
use snowflake::generator::SafeIdGenerator;

use service::IdGenService;
use log_layer::LogLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ipv6 baby!!!!
    let address = "[::1]:10000".parse().unwrap();

    let id_structure = IdStructure::builder()
        .timestamp_bits(41)
        .gen_id_bits(10)
        .sequence_bits(13)
        .epoch(Instant::now())
        .create();

    // IdStructure is Copy, so we dont lose it when we copy it into .structure here:
    let generator = Arc::new(SafeIdGenerator::builder()
        .structure(id_structure)
        .id(0)
        .create());

    // the grpc service needs a safe reference to the actual generator impl
    let service = IdGenService::new(generator.clone());

    // this is the actual grpc "server" service which is passed to tonic's transport layer. You can
    // have multiple of these per transport server, similar to an ASP.NET Controller.
    let server = IdGenServer::new(service);

    // this is the actual transport server being built. Its services are grpc servers being exposed
    // and served on the address passed. Layers are just middleware. DI doesnt exist in the same
    // reflection-based auto-injection capacity as in ASP.NET. Instead, we can setup an application
    // wide cache or just inject directly as we do above. In fact, IoC is considered an
    // anti-pattern! For direct DI, read on the type-state pattern, and the builder pattern.
    Server::builder()
        .layer(LogLayer::new("idgen"))
        .add_service(server)
        .serve(address).await?;

    Ok(())
}
