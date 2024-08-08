pub mod config;
mod dictionaries;
mod kanjis;
mod kanjis_meta;
mod tags;
pub mod terms;
mod terms_meta;

use surrealdb::{
    engine::local::{Db, RocksDb},
    Result, Surreal,
};

use crate::dict::LoadedDict;

pub struct Database {
    pub conn: Surreal<Db>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        // TODO: WASM - use indexed db instead
        let conn = surrealdb::Surreal::new::<RocksDb>("s_database.db")
            .await
            .unwrap();
        conn.use_ns("yomishi").use_db("yomishi").await.unwrap();

        // TODO: do not re-create it every time, slows down startup a lot
        conn.query(include_str!("database/create.surql"))
            .await
            .unwrap();

        Ok(Self { conn })
    }

    pub async fn load(&mut self, dictionary: LoadedDict) -> Result<()> {
        let (index, term, term_meta, kanji, kanji_meta, tag) = dictionary;

        if self.dict_exists(&index).await? {
            return Ok(());
        }

        let dictionary_id = self.insert_dictionary(&index).await?;
        self.insert_tags_bulk(tag, &dictionary_id).await?;

        self.insert_terms_bulk(term, &dictionary_id).await?;
        self.insert_terms_meta_bulk(term_meta, &dictionary_id)
            .await?;

        self.insert_kanjis_bulk(kanji, &dictionary_id);
        self.insert_kanjis_meta_bulk(kanji_meta, &dictionary_id);
        Ok(())
    }
}
