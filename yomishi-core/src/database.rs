mod dictionaries;
mod kanjis;
mod kanjis_meta;
mod tags;
mod terms;
mod terms_meta;

use std::path::Path;

use rusqlite::{params, Connection};

use crate::{
    deinflector::{DeinflectionMeta, DeinflectionResult, Deinflector},
    dict::{parser::term::Term, DictIndex, LoadedDict},
};

use self::{
    dictionaries::{dict_exists, insert_dictionary},
    kanjis::insert_kanjis_bulk,
    kanjis_meta::insert_kanjis_meta_bulk,
    tags::insert_tags_bulk,
    terms::insert_terms_bulk,
    terms_meta::insert_terms_meta_bulk,
};

#[derive(Debug)]
pub struct SearchResult<'a>(pub Term, pub DeinflectionMeta<'a>);

pub struct Database {
    conn: Connection,
    deinflector: Deinflector,
}

impl Database {
    pub fn new(deinflector: Deinflector) -> rusqlite::Result<Self> {
        let conn = Connection::open(Path::new("./db.sqlite3"))?;

        conn.execute_batch(concat!(
            "PRAGMA journal_mode = WAL;",
            include_str!("database/create.sql"),
            include_str!("database/index.sql")
        ))?;

        Ok(Self { conn, deinflector })
    }

    pub fn dict_exists(&self, index: &DictIndex) -> rusqlite::Result<bool> {
        dict_exists(&self.conn, index)
    }

    pub fn load(&mut self, dictionary: LoadedDict) -> rusqlite::Result<()> {
        let (index, term, term_meta, kanji, kanji_meta, tag) = dictionary;

        if dict_exists(&self.conn, &index)? {
            return Ok(());
        }

        let tx = self.conn.transaction()?;

        let dictionary_id = insert_dictionary(&tx, &index)?;

        insert_terms_bulk(&tx, term, dictionary_id)?;
        insert_terms_meta_bulk(&tx, term_meta, dictionary_id)?;

        insert_kanjis_bulk(&tx, kanji, dictionary_id)?;
        insert_kanjis_meta_bulk(&tx, kanji_meta, dictionary_id)?;

        insert_tags_bulk(&tx, tag, dictionary_id)?;

        tx.commit()
    }

    pub fn search<'a>(
        &'a mut self,
        text: &'a str,
    ) -> rusqlite::Result<Vec<(DeinflectionMeta, Vec<Term>)>> {
        let deinf = self.deinflector.deinflect(text);
        let mut s_sql = self.conn.prepare(
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

        deinf
            .into_iter()
            .map(|DeinflectionResult(term, meta)| {
                Ok((
                    meta,
                    s_sql
                        .query_map(params![&term], |e| {
                            Ok(Term {
                                expression: e.get(0)?,
                                reading: e.get(1)?,
                                definition_tags: e.get(2)?,
                                rules: e.get(3)?,
                                score: e.get(4)?,
                                glossary: serde_json::from_str(&e.get::<_, String>(5)?).unwrap(),
                                sequence: e.get(6)?,
                                term_tags: e.get(7)?,
                            })
                        })?
                        .collect::<rusqlite::Result<_>>()?,
                ))
            })
            .collect()
    }
}
