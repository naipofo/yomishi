use rusqlite::Connection;

use crate::dict::DictIndex;

pub fn dict_exists(conn: &Connection, index: &DictIndex) -> rusqlite::Result<bool> {
    Ok(conn
        .prepare("SELECT EXISTS(SELECT 1 FROM dictionaries WHERE title = ? AND revision = ?)")?
        .query_row(index_to_touple(&index), |r| r.get::<_, i64>(0))?
        == 1)
}

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
