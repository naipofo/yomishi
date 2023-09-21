mod dictionaries;
mod kanjis;
mod kanjis_meta;
mod tags;
mod terms;
mod terms_meta;

use rusqlite::Connection;
use std::path::Path;

use crate::dict::LoadedDict;

use self::{
    dictionaries::insert_dictionary, kanjis::insert_kanjis_bulk,
    kanjis_meta::insert_kanjis_meta_bulk, tags::insert_tags_bulk, terms::insert_terms_bulk,
    terms_meta::insert_terms_meta_bulk,
};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new() -> rusqlite::Result<Self> {
        let conn = Connection::open(Path::new("./db.sqlite3"))?;

        conn.execute_batch(concat!(
            "PRAGMA journal_mode = WAL;",
            include_str!("database/create.sql"),
            include_str!("database/index.sql")
        ))?;

        Ok(Self { conn })
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
}
