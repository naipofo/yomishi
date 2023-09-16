use crate::{
    database::Database,
    deinflector::Deinflector,
    dict::import_from_directory,
    protos::yomishi::{anki::anki_server::AnkiServer, scan::scan_server::ScanServer},
    services::{anki::AnkiService, scan::ScanService},
};
use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;
use tonic::transport::Server;

mod anki_connect;
mod database;
mod deinflector;
mod dict;
mod japanese;
mod protos;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, yomishi!");

    let mut db = Database::new(
        Deinflector::new_from_str(include_str!("../../local_test_files/deinflect.json")).unwrap(),
    )
    .unwrap();

    let dicts = import_from_directory(Path::new("../local_test_files/dic"), |index| {
        !db.dict_exists(index).unwrap()
    })
    .unwrap();
    for d in dicts {
        db.load(d).unwrap();
    }
    println!("loaded all!");

    let db = Arc::new(Mutex::new(db));
    let addr = "[::1]:50051".parse()?;

    let scan_service = ScanService { db: db.clone() };
    let anki_service = AnkiService { db: db.clone() };

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(ScanServer::new(scan_service)))
        .add_service(tonic_web::enable(AnkiServer::new(anki_service)))
        .serve(addr)
        .await?;

    Ok(())
}
