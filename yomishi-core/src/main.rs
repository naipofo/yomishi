use crate::{
    backend::Backend,
    protos::yomishi::{
        anki::anki_server::AnkiServer, config::config_service_server::ConfigServiceServer,
        scan::scan_server::ScanServer,
    },
};
use std::sync::Arc;
use tonic::transport::Server;

mod anki_connect;
mod backend;
mod database;
mod deinflector;
mod dict;
mod dictionary;
mod error;
mod flashcard;
mod html;
mod japanese;
mod protos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, yomishi!");

    let backend = Arc::new(Backend::new().await);
    println!("Loaded all");

    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(ScanServer::from_arc(Arc::clone(
            &backend,
        ))))
        .add_service(tonic_web::enable(AnkiServer::from_arc(Arc::clone(
            &backend,
        ))))
        .add_service(tonic_web::enable(ConfigServiceServer::from_arc(
            Arc::clone(&backend),
        )))
        .serve(addr)
        .await?;

    Ok(())
}
