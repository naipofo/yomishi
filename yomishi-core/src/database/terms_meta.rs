use rusqlite::{params, Connection};

use crate::dict::parser::term_meta::TermMeta;

pub fn insert_terms_meta_bulk(
    conn: &Connection,
    terms: Vec<TermMeta>,
    dictionary_id: i64,
) -> rusqlite::Result<()> {
    let mut prep = conn.prepare_cached(
        "INSERT INTO terms_meta(
            term,
            reading,
            entry,
            dictionary
        ) VALUES (?, ?, ?, ?)",
    )?;

    terms
        .iter()
        .map(|t| {
            prep.insert(params![
                t.term,
                t.reading,
                serde_json::to_string(&t.entry).unwrap(),
                dictionary_id
            ])
            .map(|_| ())
        })
        .collect::<rusqlite::Result<_>>()?;
    prep.discard();
    Ok(())
}
