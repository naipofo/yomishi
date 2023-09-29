use rusqlite::{params, Connection};

use crate::dict::parser::tag::Tag;

use super::Database;

pub fn insert_tags_bulk(
    conn: &Connection,
    terms: Vec<Tag>,
    dictionary_id: i64,
) -> rusqlite::Result<()> {
    let mut prep = conn.prepare_cached(
        "INSERT INTO tags(
            name,
            category,
            sorting,
            notes,
            popularity,
            dictionary
        ) VALUES (?, ?, ?, ?, ?, ?)",
    )?;

    terms.iter().try_for_each(|t| {
        prep.insert(params![
            t.name,
            t.category,
            t.sorting,
            t.notes,
            t.popularity,
            dictionary_id
        ])
        .map(|_| ())
    })?;
    prep.discard();
    Ok(())
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
    pub fn get_tag_list(&self, names: &[String], dict_id: &i64) -> rusqlite::Result<Vec<Tag>> {
        names.iter().map(|e| self.get_tag(e, dict_id)).collect()
    }
}
