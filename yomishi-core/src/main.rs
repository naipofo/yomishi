use crate::{
    database::{slow_inmem::SlowInMemeoryDatabase, Database},
    deinflector::Deinflector,
    dict_parser::import_from_path,
    protos::yomishi::scan::scan_server::ScanServer,
    services::ScanService,
};
use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;
use tonic::transport::Server;

mod database;
mod deinflector;
mod dict_parser;
mod japanese;
mod protos;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, yomishi!");

    let mut db = SlowInMemeoryDatabase::new(
        Deinflector::new_from_str(include_str!("../../local_test_files/deinflect.json")).unwrap(),
    );

    let (title, dict) = import_from_path(Path::new("../local_test_files/index.json")).unwrap();
    db.load(title, dict);

    let db = Arc::new(Mutex::new(db));
    let addr = "[::1]:50051".parse()?;

    let scan_service = ScanService { db: db.clone() };

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(ScanServer::new(scan_service)))
        .serve(addr)
        .await?;

    Ok(())
}
