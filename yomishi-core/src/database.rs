mod dictionaries;
mod kanjis;
mod kanjis_meta;
mod tags;
mod terms;
mod terms_meta;

use std::{collections::BTreeMap, path::Path, vec};

use rusqlite::{params, Connection};

use crate::{
    deinflector::{DeinflectionMeta, DeinflectionResult, Deinflector},
    dict::{
        parser::{tag::Tag, term::Term, term_meta::TermMeta},
        DictIndex, LoadedDict,
    },
};

use self::{
    dictionaries::{dict_exists, insert_dictionary},
    kanjis::insert_kanjis_bulk,
    kanjis_meta::insert_kanjis_meta_bulk,
    tags::insert_tags_bulk,
    terms::insert_terms_bulk,
    terms_meta::insert_terms_meta_bulk,
};

pub struct Database {
    pub conn: Connection,
    deinflector: Deinflector,
}

pub struct SearchResult<'a> {
    pub deinflection: DeinflectionMeta<'a>,
    pub glossares: Vec<(Term, Vec<TermMeta>, Vec<Tag>)>,
    pub tags: Vec<Tag>,
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

    pub fn search<'a>(&'a mut self, text: &'a str) -> rusqlite::Result<Vec<SearchResult>> {
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

        // TODO: make this not peak at 12 levels of indent

        Ok(deinf
            .into_iter()
            .map(|DeinflectionResult(term, meta)| {
                struct LookupResult(Term, Vec<TermMeta>, Vec<Tag>, Vec<Tag>);

                #[derive(PartialEq, Eq, PartialOrd, Ord)]
                struct DedupKey(String, String);

                fn group_pairs(v: Vec<LookupResult>) -> BTreeMap<DedupKey, Vec<LookupResult>> {
                    v.into_iter().fold(BTreeMap::new(), |mut acc, el| {
                        acc.entry(DedupKey(
                            el.0.expression.to_string(),
                            el.0.reading.to_string(),
                        ))
                        .or_default()
                        .push(el);
                        acc
                    })
                }

                let terms_r: Vec<LookupResult> = s_sql
                    .query_map(
                        params![&term],
                        |e| -> Result<LookupResult, rusqlite::Error> {
                            let term = Term {
                                expression: e.get(0)?,
                                reading: e.get(1)?,
                                definition_tags: serde_json::from_str(&e.get::<_, String>(2)?)
                                    .unwrap(),
                                rules: e.get(3)?,
                                score: e.get(4)?,
                                glossary: serde_json::from_str(&e.get::<_, String>(5)?).unwrap(),
                                sequence: e.get(6)?,
                                term_tags: serde_json::from_str(&e.get::<_, String>(7)?).unwrap(),
                            };

                            let dict_id: i64 = e.get(8)?;

                            let tags = self.get_tag_list(&term.definition_tags, &dict_id)?;
                            let term_tags = self.get_tag_list(&term.term_tags, &dict_id)?;
                            Ok(LookupResult(term, Vec::<TermMeta>::new(), tags, term_tags))
                        },
                    )?
                    .collect::<rusqlite::Result<_>>()?;

                let terms_grouped = group_pairs(terms_r);

                Ok(terms_grouped
                    .into_iter()
                    .map(|(_, e)| {
                        let mut all_tags = vec![];
                        SearchResult {
                            deinflection: meta.clone(),
                            glossares: e
                                .into_iter()
                                .map(|LookupResult(term, meta, tag, global)| {
                                    all_tags.extend(global.into_iter());
                                    (term, meta, tag)
                                })
                                .collect::<Vec<_>>(),
                            tags: all_tags,
                        }
                    })
                    .collect::<Vec<_>>())
            })
            .collect::<rusqlite::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}
