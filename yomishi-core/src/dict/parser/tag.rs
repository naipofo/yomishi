use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::dict::parser::FromBank;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub name: String,
    pub category: String,
    pub sorting: i64,
    pub notes: String,
    pub popularity: i64,
}

impl FromBank for Tag {
    fn parse(r: VecDeque<Value>, _: i32) -> serde_json::Result<Self> {
        convert(r)
    }
}

fn convert(mut v: VecDeque<Value>) -> serde_json::Result<Tag> {
    Ok(Tag {
        name: serde_json::from_value(v.pop_front().unwrap())?,
        category: serde_json::from_value(v.pop_front().unwrap())?,
        sorting: serde_json::from_value(v.pop_front().unwrap())?,
        notes: serde_json::from_value(v.pop_front().unwrap())?,
        popularity: serde_json::from_value(v.pop_front().unwrap())?,
    })
}
