pub mod config;
mod dictionaries;
mod kanjis;
mod kanjis_meta;
mod tags;
mod terms;
mod terms_meta;

use rusqlite::Connection;
use std::path::Path;
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

use crate::dict::LoadedDict;

use self::{
    dictionaries::insert_dictionary, kanjis::insert_kanjis_bulk,
    kanjis_meta::insert_kanjis_meta_bulk, tags::insert_tags_bulk, terms::insert_terms_bulk,
    terms_meta::insert_terms_meta_bulk,
};

pub struct Database {
    pub conn: Connection,
    pub s_conn: Surreal<Db>,
}

impl Database {
    pub async fn new() -> rusqlite::Result<Self> {
        // TODO: WASM - use indexed db instead
        let s_conn = surrealdb::Surreal::new::<Mem>(()).await.unwrap();
        s_conn.use_ns("yomishi").use_db("yomishi").await.unwrap();
        s_conn
            .query(include_str!("database/create.surql"))
            .await
            .unwrap();

        let conn = Connection::open(Path::new("./db.sqlite3"))?;
        conn.execute_batch(concat!(
            "PRAGMA journal_mode = WAL;",
            include_str!("database/create.sql"),
            include_str!("database/index.sql")
        ))?;

        Ok(Self { conn, s_conn })
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
