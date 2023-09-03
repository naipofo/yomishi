use rusqlite::{params, Connection};

use crate::dict::parser::tag::Tag;

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

    terms
        .iter()
        .map(|t| {
            prep.insert(params![
                t.name,
                t.category,
                t.sorting,
                t.notes,
                t.popularity,
                dictionary_id
            ])
            .map(|_| ())
        })
        .collect::<rusqlite::Result<_>>()?;
    prep.discard();
    Ok(())
}
