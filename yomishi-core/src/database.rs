mod dictionaries;
mod kanjis;
mod kanjis_meta;
mod tags;
mod terms;
mod terms_meta;

use std::{
    collections::{BTreeMap, HashSet},
    path::Path,
};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::{
    deinflector::{DeinflectionMeta, DeinflectionResult, Deinflector},
    dict::{
        parser::{tag::Tag, term::Term, term_meta::TermMeta},
        LoadedDict,
    },
};

use self::{
    dictionaries::insert_dictionary, kanjis::insert_kanjis_bulk,
    kanjis_meta::insert_kanjis_meta_bulk, tags::insert_tags_bulk, terms::insert_terms_bulk,
    terms_meta::insert_terms_meta_bulk,
};

pub struct Database {
    pub conn: Connection,
    deinflector: Deinflector,
}

#[derive(Deserialize, Serialize)]
pub struct SearchResult<'a> {
    #[serde(borrow)]
    pub deinflection: DeinflectionMeta<'a>,
    pub glossaries: Vec<TermWithTags>,
    pub tags: Vec<Tag>,
    pub meta: Vec<DictionaryTagged<TermMeta>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DictionaryTagged<T> {
    pub dictionary: String,
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TermWithTags {
    pub term: Term,
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

    pub fn load(&mut self, dictionary: LoadedDict) -> rusqlite::Result<()> {
        let (index, term, term_meta, kanji, kanji_meta, tag) = dictionary;

        if self.dict_exists(&index)? {
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
        self.deinflector
            .deinflect(text)
            .into_iter()
            .map(|DeinflectionResult(term, meta)| {
                struct LookupResult(Term, Vec<Tag>, Vec<Tag>);

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

                let terms_r = self
                    .get_terms(&term)?
                    .into_iter()
                    .map(|(term, dict_id)| {
                        let tags = self.get_tag_list(&term.definition_tags, &dict_id)?;
                        let term_tags = self.get_tag_list(&term.term_tags, &dict_id)?;

                        Ok(LookupResult(term, tags, term_tags))
                    })
                    .collect::<rusqlite::Result<_>>()?;

                let terms_grouped = group_pairs(terms_r);

                terms_grouped
                    .into_iter()
                    .map(|(_, e)| {
                        let mut all_tags = HashSet::new();

                        let t = &e.get(0).unwrap().0;
                        let term_meta = self.get_term_meta(&t.expression, &t.reading)?;
                        Ok(SearchResult {
                            deinflection: meta.clone(),
                            glossaries: e
                                .into_iter()
                                .map(|LookupResult(term, tags, global)| {
                                    all_tags.extend(global.into_iter());
                                    TermWithTags { term, tags }
                                })
                                .collect::<Vec<_>>(),
                            tags: all_tags.into_iter().collect(),
                            meta: term_meta,
                        })
                    })
                    .collect::<rusqlite::Result<Vec<_>>>()
            })
            .collect::<rusqlite::Result<Vec<_>>>()
            .map(|e| e.into_iter().flatten().collect())
    }
}
