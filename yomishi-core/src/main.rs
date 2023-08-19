use std::path::Path;

use crate::{
    database::{slow_inmem::SlowInMemeoryDatabase, Database},
    dict_parser::import_from_path,
};

mod database;
mod deinflector;
mod dict_parser;

fn main() {
    println!("Hello, yomishi!");

    let (title, dict) = import_from_path(Path::new("../local_test_files/index.json")).unwrap();
    println!("imported \"{title}\" with {} terms", dict.len());

    let mut db = SlowInMemeoryDatabase::new(
        serde_json::from_str(include_str!("../../local_test_files/deinflect.json")).unwrap(),
    );

    db.load(title, dict);

    for r in db.search("食べさせない") {
        println!("{:?}", r);
    }
}
