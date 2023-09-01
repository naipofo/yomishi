use std::{
    fs::{read_dir, File},
    path::Path,
};

use regex::Regex;
use serde::Deserialize;

use self::parser::{parse_bank, term::Term};

pub mod html;
pub mod parser;

#[derive(Debug, Deserialize)]
struct DictIndex {
    title: String,
    format: i32,
}

pub fn import_from_directory(dir_path: &Path) -> std::io::Result<Vec<(String, Vec<Term>)>> {
    let re = Regex::new(r"term\_bank\_(\d+)\.json").unwrap();

    read_dir(dir_path)?
        .into_iter()
        .map(|e| {
            let mut zip = zip::ZipArchive::new(File::open(e?.path())?)?;
            let DictIndex { title, format } =
                serde_json::from_reader(zip.by_name("index.json")?).unwrap();
            let terms = zip
                .file_names()
                .filter(|e| re.is_match(e))
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .into_iter()
                .map(|e| parse_bank(format, zip.by_name(&e).unwrap()))
                .flatten()
                .collect();
            Ok((title, terms))
        })
        .collect::<Result<Vec<(String, Vec<Term>)>, _>>()
}
