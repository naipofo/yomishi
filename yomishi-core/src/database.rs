use std::path::Path;

use rusqlite::{params, Connection};

use crate::{
    deinflector::{DeinflectionMeta, DeinflectionResult, Deinflector},
    dict::{parser::term::Term, DictIndex},
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

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dictionaries(
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                revision TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS terms(
                id INTEGER PRIMARY KEY,
                expression TEXT NOT NULL,
                reading TEXT NOT NULL,
                definition_tags TEXT,
                rules TEXT NOT NULL,
                score INTEGER NOT NULL,
                glossary TEXT NOT NULL,
                sequence INTEGER NOT NULL,
                term_tags TEXT NOT NULL,
                dictionary INTEGER NOT NULL,
                FOREIGN KEY(dictionary) REFERENCES dictionaries(id)
            );
            PRAGMA journal_mode = WAL;",
        )?;

        Ok(Self { conn, deinflector })
    }

    pub fn load(&mut self, index: &DictIndex, terms: Vec<Term>) -> rusqlite::Result<()> {
        if self
            .conn
            .prepare("SELECT EXISTS(SELECT 1 FROM dictionaries WHERE title = ? AND revision = ?)")?
            .query_row(index_to_touple(&index), |r| r.get::<_, i64>(0))?
            == 1
        {
            return Ok(());
        }

        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT INTO dictionaries(
                title,
                revision
            ) VALUES (?, ?)",
            index_to_touple(&index),
        )?;
        let d_id = tx.last_insert_rowid();

        let mut prep = tx.prepare_cached(
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
                    d_id
                ])
                .map(|_| ())
            })
            .collect::<rusqlite::Result<_>>()?;
        prep.discard();
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

fn index_to_touple(i: &DictIndex) -> (&str, &str) {
    (&i.revision, &i.title)
}
