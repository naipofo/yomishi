use std::collections::VecDeque;

use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{database::Database, dict::parser::FromBank};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Tag {
    pub name: String,
    pub category: String,
    pub sorting: i64,
    pub notes: String,
    pub popularity: i64,
}

impl FromBank for Tag {
    fn parse(r: VecDeque<Value>, _: i64) -> serde_json::Result<Self> {
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

impl Database {
    fn get_tag(&self, name: &str, dict_id: &i64) -> rusqlite::Result<Tag> {
        self.conn
            .prepare(
                "SELECT
                    name,
                    category,
                    sorting,
                    notes,
                    popularity
                FROM tags 
                WHERE name = ? AND dictionary = ?",
            )?
            .query_row(params![name, dict_id], |tag_row| {
                Ok(Tag {
                    name: tag_row.get(0)?,
                    category: tag_row.get(1)?,
                    sorting: tag_row.get(2)?,
                    notes: tag_row.get(3)?,
                    popularity: tag_row.get(4)?,
                })
            })
    }
    pub fn get_tag_list(&self, names: &Vec<String>, dict_id: &i64) -> rusqlite::Result<Vec<Tag>> {
        names.iter().map(|e| self.get_tag(e, dict_id)).collect()
    }
}
