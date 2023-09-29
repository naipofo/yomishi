use rusqlite::{params, Connection};

use crate::{dict::parser::term_meta::TermMeta, scan::search::DictionaryTagged};

use super::Database;

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

impl Database {
    pub fn get_term_meta(
        &self,
        term: &str,
        reading: &str,
    ) -> rusqlite::Result<Vec<DictionaryTagged<TermMeta>>> {
        let mut prep = self.conn.prepare(
            "SELECT
                term,
                reading,
                entry,
                dictionary
                FROM terms_meta 
            WHERE term = ?",
        )?;
        let results = prep
            .query_map(params![term], |e| {
                Ok((
                    TermMeta {
                        term: e.get(0)?,
                        reading: e.get(1)?,
                        entry: serde_json::from_str(&e.get::<_, String>(2)?).unwrap(),
                    },
                    e.get(3)?,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?
            .into_iter()
            .flat_map(|(e, dict_id)| match &e.reading {
                // TODO: nicer filter
                Some(r) => {
                    if *r == reading {
                        Some(self.get_dict_by_id(&dict_id).map(|d| (d, e)))
                    } else {
                        None
                    }
                }
                None => Some(self.get_dict_by_id(&dict_id).map(|d| (d, e))),
            })
            .collect::<rusqlite::Result<Vec<_>>>()?
            .into_iter()
            .map(|(dictionary, data)| DictionaryTagged { dictionary, data })
            .collect();

        Ok(results)
    }
}
