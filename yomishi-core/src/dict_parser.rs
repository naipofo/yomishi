use std::collections::VecDeque;

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

pub fn parse_dict(index: &str, bank: &str) -> (String, Vec<Term>) {
    // TODO: parse other categories besides terms
    let DictIndex { title, format } = serde_json::from_str(index).unwrap();
    // TODO: convert from v1
    if format != 3 {
        todo!("No support for format other than 3")
    }
    let entries: Vec<VecDeque<Value>> = serde_json::from_str(bank).unwrap();

    (
        title,
        entries
            .into_iter()
            .map(convert_term_v3)
            .collect::<Result<_, _>>()
            .unwrap(),
    )
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
