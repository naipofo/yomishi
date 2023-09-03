use rusqlite::{params, Connection};

use crate::dict::parser::term::Term;

pub fn insert_terms_bulk(
    conn: &Connection,
    terms: Vec<Term>,
    dictionary_id: i64,
) -> rusqlite::Result<()> {
    let mut prep = conn.prepare_cached(
        "INSERT INTO terms(
            expression,
            reading,
            definition_tags,
            rules,
            score,
            glossary,
            sequence,
            term_tags,
            dictionary
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )?;

    terms
        .iter()
        .map(|t| {
            prep.insert(params![
                t.expression,
                t.reading,
                t.definition_tags,
                t.rules,
                t.score,
                serde_json::to_string(&t.glossary).unwrap(),
                t.sequence,
                t.term_tags,
                dictionary_id
            ])
            .map(|_| ())
        })
        .collect::<rusqlite::Result<_>>()?;
    prep.discard();
    Ok(())
}
