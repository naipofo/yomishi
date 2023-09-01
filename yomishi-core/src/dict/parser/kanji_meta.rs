use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::dict::parser::FromBank;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KanjiMeta {
    pub kanji: String,
    pub value: MetaValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetaValue {
    value: i64,
    display_value: String,
}

impl FromBank for KanjiMeta {
    fn parse(r: VecDeque<Value>, _: i32) -> serde_json::Result<Self> {
        convert(r)
    }
}

fn convert(mut v: VecDeque<Value>) -> serde_json::Result<KanjiMeta> {
    Ok(KanjiMeta {
        kanji: serde_json::from_value(v.pop_front().unwrap())?,
        value: serde_json::from_value(v.pop_back().unwrap())?,
    })
}
