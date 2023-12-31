use std::collections::VecDeque;

use super::{structured::StructuredContent, FromBank};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{formats::SpaceSeparator, serde_as, DeserializeAs, StringWithSeparator};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Term {
    pub expression: String,
    pub reading: String,
    pub definition_tags: Vec<String>,
    pub rules: String,
    pub score: i64,
    pub glossary: Vec<GlossaryEntry>,
    pub sequence: i64,
    pub term_tags: Vec<String>,
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

impl FromBank for Term {
    fn parse(r: VecDeque<Value>, format: i64) -> serde_json::Result<Self> {
        (if format == 1 { convert_v1 } else { convert_v3 })(r).map(fill_reading)
    }
}

fn convert_v3(mut v: VecDeque<Value>) -> serde_json::Result<Term> {
    Ok(Term {
        expression: serde_json::from_value(v.pop_front().unwrap())?,
        reading: serde_json::from_value(v.pop_front().unwrap())?,
        definition_tags: StringWithSeparator::<SpaceSeparator, String>::deserialize_as(
            v.pop_front().unwrap(),
        )?,
        rules: serde_json::from_value(v.pop_front().unwrap())?,
        score: serde_json::from_value(v.pop_front().unwrap())?,
        glossary: serde_json::from_value(v.pop_front().unwrap())?,
        sequence: serde_json::from_value(v.pop_front().unwrap())?,
        term_tags: StringWithSeparator::<SpaceSeparator, String>::deserialize_as(
            v.pop_front().unwrap(),
        )?,
    })
}

fn convert_v1(mut v: VecDeque<Value>) -> serde_json::Result<Term> {
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
        term_tags: vec![],
    })
}

fn fill_reading(t: Term) -> Term {
    if !t.reading.is_empty() {
        t
    } else {
        Term {
            reading: t.expression.clone(),
            ..t
        }
    }
}
