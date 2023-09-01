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
    Frequency(i64),
    Pitches(Vec<Pitch>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pitch {
    position: i64,
    nasal: Option<Vec<i64>>,
    devoice: Option<Vec<i64>>,
    tags: Vec<String>,
}

impl FromBank for TermMeta {
    fn parse(r: VecDeque<Value>, _: i32) -> serde_json::Result<Self> {
        convert(r)
    }
}

fn convert(mut v: VecDeque<Value>) -> serde_json::Result<TermMeta> {
    let term = from_value(v.pop_front().unwrap())?;

    Ok(match v.pop_back().unwrap() {
        Value::Number(n) => TermMeta {
            term,
            reading: None,
            entry: TermMetaEntry::Frequency(n.as_i64().unwrap()),
        },
        Value::Object(mut o) => {
            let reading = o
                .remove("reading")
                .and_then(|e| e.as_str().map(|e| e.to_string()));

            TermMeta {
                term,
                reading,
                entry: match o.get("frequency") {
                    Some(f) => TermMetaEntry::Frequency(f.as_i64().unwrap()),
                    None => {
                        let pitches: Vec<Value> = from_value(o.remove("pitches").unwrap())?;

                        TermMetaEntry::Pitches(
                            pitches
                                .into_iter()
                                .map(|mut e| e.as_object_mut().and_then(|e| parse_pitch(e)))
                                .collect::<Option<_>>()
                                .unwrap(),
                        )
                    }
                },
            }
        }
        _ => panic!(),
    })
}

fn parse_pitch(v: &mut Map<String, Value>) -> Option<Pitch> {
    Some(Pitch {
        position: v.remove("position")?.as_i64()?,
        nasal: v.remove("nasal").and_then(|e| num_or_array(e).ok()),
        devoice: v.remove("devoice").and_then(|e| num_or_array(e).ok()),
        tags: v
            .remove("tags")
            .and_then(|e| from_value(e).ok())
            .unwrap_or(vec![]),
    })
}

fn num_or_array(v: Value) -> serde_json::Result<Vec<i64>> {
    Ok(match v {
        Value::Number(n) => vec![n.as_i64().unwrap()],
        Value::Array(a) => from_value(serde_json::Value::Array(a))?,
        _ => panic!(),
    })
}
