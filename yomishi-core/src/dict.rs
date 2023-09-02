use std::{
    collections::HashMap,
    fs::{read_dir, File},
    path::Path,
};

use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

use self::parser::{parse_bank, term::Term};

pub mod html;
pub mod parser;

#[derive(Debug, Deserialize)]
pub struct DictIndex {
    pub title: String,
    pub revision: String,
    format: i64,
}

pub fn import_from_directory(dir_path: &Path) -> std::io::Result<Vec<(DictIndex, Vec<Term>)>> {
    let re = Regex::new(r"term\_bank\_(\d+)\.json").unwrap();

    read_dir(dir_path)?
        .into_iter()
        .map(|e| {
            let mut zip = zip::ZipArchive::new(File::open(e?.path())?)?;

            let mut ob: HashMap<String, Value> =
                serde_json::from_reader(zip.by_name("index.json")?).unwrap();
            let index = DictIndex {
                title: ob.remove("title").unwrap().to_string(),
                revision: ob.remove("revision").unwrap().to_string(),
                format: ob
                    .get("version")
                    .unwrap_or(ob.get("format").unwrap())
                    .as_i64()
                    .unwrap(),
            };

            let terms = zip
                .file_names()
                .filter(|e| re.is_match(e))
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .into_iter()
                .map(|e| {
                    parse_bank(
                        i32::try_from(index.format).unwrap(),
                        zip.by_name(&e).unwrap(),
                    )
                })
                .flatten()
                .collect();
            Ok((index, terms))
        })
        .collect()
}
