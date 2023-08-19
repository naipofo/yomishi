use std::{
    collections::VecDeque,
    fs::{read_dir, read_to_string},
    path::Path,
};

use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct DictIndex {
    title: String,
    format: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    expression: String,
    reading: String,
    definition_tags: String,
    rules: String,
    score: u64,
    glossary: Vec<String>,
    sequence: u64,
    term_tags: String,
}

fn parse_bank(format: i32, bank: &str) -> Vec<Term> {
    // TODO: convert from v1
    if format != 3 {
        todo!("No support for format other than 3")
    }

    serde_json::from_str::<Vec<_>>(bank)
        .unwrap()
        .into_iter()
        .map(convert_term_v3)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn convert_term_v3(v: VecDeque<Value>) -> serde_json::Result<Term> {
    let mut v = v;
    Ok(Term {
        expression: serde_json::from_value(v.pop_front().unwrap())?,
        reading: serde_json::from_value(v.pop_front().unwrap())?,
        definition_tags: serde_json::from_value(v.pop_front().unwrap())?,
        rules: serde_json::from_value(v.pop_front().unwrap())?,
        score: serde_json::from_value(v.pop_front().unwrap())?,
        glossary: serde_json::from_value(v.pop_front().unwrap())?,
        sequence: serde_json::from_value(v.pop_front().unwrap())?,
        term_tags: serde_json::from_value(v.pop_front().unwrap())?,
    })
}

pub fn import_from_path(index_path: &Path) -> std::io::Result<(String, Vec<Term>)> {
    let index = read_to_string(index_path)?;
    let DictIndex { title, format } = serde_json::from_str(&index).unwrap();

    let files = read_dir(index_path.parent().unwrap())?;
    let re = Regex::new(r"term\_bank\_(\d+)\.json").unwrap();

    let terms = files
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| re.is_match(&e.file_name().to_string_lossy()))
        .map(|f| Ok(parse_bank(format, &read_to_string(f.path())?)))
        .collect::<Result<Vec<_>, std::io::Error>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok((title, terms))
}
