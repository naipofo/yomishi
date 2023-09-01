use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::FromBank;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kanji {
    pub character: String,
    pub onyomi: String, // TODO: Space separated
    pub kunyomi: String,
    kanji_tags: String,
    pub meaning: Vec<String>,
    various: HashMap<String, String>,
}

impl FromBank for Kanji {
    fn parse(r: VecDeque<Value>, format: i32) -> serde_json::Result<Self> {
        (if format == 1 { convert_v1 } else { convert_v3 })(r)
    }
}

fn convert_v1(mut v: VecDeque<Value>) -> serde_json::Result<Kanji> {
    Ok(Kanji {
        character: serde_json::from_value(v.pop_front().unwrap())?,
        onyomi: serde_json::from_value(v.pop_front().unwrap())?,
        kunyomi: serde_json::from_value(v.pop_front().unwrap())?,
        kanji_tags: serde_json::from_value(v.pop_front().unwrap())?,
        meaning: vec![match v.pop_front() {
            Some(v) => serde_json::from_value(v)?,
            None => "".to_string(),
        }],
        various: HashMap::new(),
    })
}

fn convert_v3(mut v: VecDeque<Value>) -> serde_json::Result<Kanji> {
    Ok(Kanji {
        character: serde_json::from_value(v.pop_front().unwrap())?,
        onyomi: serde_json::from_value(v.pop_front().unwrap())?,
        kunyomi: serde_json::from_value(v.pop_front().unwrap())?,
        kanji_tags: serde_json::from_value(v.pop_front().unwrap())?,
        meaning: serde_json::from_value(v.pop_front().unwrap())?,
        various: serde_json::from_value(v.pop_front().unwrap())?,
    })
}
