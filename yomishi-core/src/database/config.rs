use prost::Message;
use rusqlite::params;

use crate::{error::Result, protos::yomishi::config::Config};

use super::Database;

impl Database {
    pub fn set_config(&self, proto: &Config) -> Result<()> {
        self.conn
            .prepare(
                "INSERT INTO config(
                    proto
                ) VALUES (?)",
            )?
            .execute(params![&proto.encode_to_vec()])?;
        Ok(())
    }

    pub fn get_config(&self) -> Result<Config> {
        Ok(Config::decode(
            &*self
                .conn
                .prepare(
                    "SELECT proto
                        FROM config
                        ORDER BY id DESC
                        LIMIT 1;",
                )?
                .query_row([], |row| row.get::<_, Vec<u8>>(0))?,
        )?)
    }
}
