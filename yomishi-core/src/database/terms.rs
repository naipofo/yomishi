use rusqlite::{params, Connection};

use crate::dict::parser::term::Term;

use super::Database;

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

    terms.iter().try_for_each(|t| {
        prep.insert(params![
            t.expression,
            t.reading,
            serde_json::to_string(&t.definition_tags).unwrap(),
            t.rules,
            t.score,
            serde_json::to_string(&t.glossary).unwrap(),
            t.sequence,
            serde_json::to_string(&t.term_tags).unwrap(),
            dictionary_id
        ])
        .map(|_| ())
    })?;
    prep.discard();
    Ok(())
}

impl Database {
    pub fn get_terms(&self, term: &str) -> rusqlite::Result<Vec<(Term, i64)>> {
        let mut prep = self.conn.prepare(
            "SELECT
                expression,
                reading,
                definition_tags,
                rules,
                score,
                glossary,
                sequence,
                term_tags,
                dictionary
                FROM terms 
            WHERE expression = ?",
        )?;
        let results = prep
            .query_map(params![term], |e| {
                Ok((
                    Term {
                        expression: e.get(0)?,
                        reading: e.get(1)?,
                        definition_tags: serde_json::from_str(&e.get::<_, String>(2)?).unwrap(),
                        rules: e.get(3)?,
                        score: e.get(4)?,
                        glossary: serde_json::from_str(&e.get::<_, String>(5)?).unwrap(),
                        sequence: e.get(6)?,
                        term_tags: serde_json::from_str(&e.get::<_, String>(7)?).unwrap(),
                    },
                    e.get(8)?,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(results)
    }
}
