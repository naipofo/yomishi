use rusqlite::{params, Connection};

use crate::dict::DictIndex;

use super::Database;

pub fn insert_dictionary(conn: &Connection, index: &DictIndex) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO dictionaries(
            title,
            revision
        ) VALUES (?, ?)",
        index_to_touple(&index),
    )?;
    Ok(conn.last_insert_rowid())
}

fn index_to_touple(i: &DictIndex) -> (&str, &str) {
    (&i.title, &i.revision)
}

impl Database {
    pub fn dict_exists(&self, index: &DictIndex) -> rusqlite::Result<bool> {
        Ok(self
            .conn
            .prepare("SELECT EXISTS(SELECT 1 FROM dictionaries WHERE title = ? AND revision = ?)")?
            .query_row(index_to_touple(&index), |r| r.get::<_, i64>(0))?
            == 1)
    }

    pub fn get_dict_by_id(&self, id: &i64) -> rusqlite::Result<String> {
        self.conn
            .prepare(
                "SELECT
                title
            FROM dictionaries
            WHERE id = ?",
            )?
            .query_row(params![id], |e| e.get(0))
    }
}
