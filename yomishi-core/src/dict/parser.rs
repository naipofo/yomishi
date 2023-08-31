pub mod structured;

use std::{collections::VecDeque, io::Read};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use self::structured::StructuredContent;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    pub expression: String,
    pub reading: String,
    definition_tags: Option<String>,
    rules: String,
    score: i64,
    pub glossary: Vec<GlossaryEntry>,
    pub sequence: i64,
    term_tags: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum GlossaryEntry {
    Text(String),
    Detailed(GlossaryDetailed),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum GlossaryDetailed {
    Text { text: String },
    Image { path: String },
    StructuredContent { content: StructuredContent },
}

pub fn parse_bank(format: i32, bank: impl Read) -> Vec<Term> {
    serde_json::from_reader::<_, Vec<_>>(bank)
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
