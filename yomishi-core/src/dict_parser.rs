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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    pub expression: String,
    pub reading: String,
    definition_tags: String,
    rules: String,
    score: u64,
    pub glossary: Vec<String>,
    sequence: u64,
    term_tags: String,
}

fn parse_bank(format: i32, bank: &str) -> Vec<Term> {
    serde_json::from_str::<Vec<_>>(bank)
        .unwrap()
        .into_iter()
        .map(if format == 1 {
            convert_term_v1
        } else {
            convert_term_v3
        })
        .map(|e| e.map(fill_reading))
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

fn convert_term_v1(v: VecDeque<Value>) -> serde_json::Result<Term> {
    let mut v = v;
    Ok(Term {
        expression: serde_json::from_value(v.pop_front().unwrap())?,
        reading: serde_json::from_value(v.pop_front().unwrap())?,
        definition_tags: serde_json::from_value(v.pop_front().unwrap())?,
        rules: serde_json::from_value(v.pop_front().unwrap())?,
        score: serde_json::from_value(v.pop_front().unwrap())?,
        glossary: v
            .into_iter()
            .map(serde_json::from_value)
            .collect::<Result<_, _>>()?,
        sequence: 0,
        term_tags: "".to_string(),
    })
}

fn fill_reading(t: Term) -> Term {
    if t.reading.len() > 0 {
        t
    } else {
        Term {
            reading: t.expression.clone(),
            ..t
        }
    }
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
