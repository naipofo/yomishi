use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Map, Value};

use crate::dict::parser::FromBank;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TermMeta {
    pub term: String,
    pub reading: Option<String>,
    pub entry: TermMetaEntry,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TermMetaEntry {
    Frequency(Option<String>, Option<i64>),
    Pitches(Vec<i64>),
}

impl FromBank for TermMeta {
    fn parse(r: VecDeque<Value>, _: i64) -> serde_json::Result<Self> {
        convert(r)
    }
}

fn convert(mut v: VecDeque<Value>) -> serde_json::Result<TermMeta> {
    let term = from_value(v.pop_front().unwrap())?;

    Ok(match v.pop_front().unwrap().as_str().unwrap() {
        "freq" => match v.pop_front().unwrap() {
            Value::Number(n) => TermMeta {
                term,
                reading: None,
                entry: TermMetaEntry::Frequency(None, n.as_i64()),
            },
            Value::String(s) => TermMeta {
                term,
                reading: None,
                entry: TermMetaEntry::Frequency(Some(s), None),
            },
            Value::Object(o) => TermMeta {
                term: term,
                reading: o
                    .get("reading")
                    .and_then(|e| e.as_str().map(|e| e.to_string())),
                entry: freq_from_object(&o),
            },
            _ => panic!(),
        },
        "pitch" => todo!(),
        _ => panic!(),
    })
}
fn freq_from_object(map: &Map<String, Value>) -> TermMetaEntry {
    TermMetaEntry::Frequency(
        map.get("displayValue")
            .and_then(|e| e.as_str().map(|e| e.to_string())),
        vec![map.get("frequency"), map.get("value")]
            .into_iter()
            .flat_map(|e| e)
            .find_map(|e| e.as_i64()),
    )
}
