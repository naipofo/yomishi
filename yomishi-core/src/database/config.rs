use prost::Message;
use rusqlite::params;
use serde_json::Value;
use yomishi_config::{BooleanKeys, IntegerKeys, StringKeys};

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

    // TODO: different result for db error / no value
    pub fn get_serde(&self, key: &str) -> Result<Value> {
        Ok(serde_json::from_str(
            &self
                .conn
                .prepare(
                    "SELECT value
                    FROM config
                    WHERE key = ?",
                )?
                .query_row([key], |row| row.get::<_, String>(0))?,
        )?)
    }

    pub fn set_serde(&self, key: &str, value: &str) -> Result<()> {
        self.conn
            .prepare(
                "INSERT OR REPLACE
                    INTO config(key, value)
                    VALUES (?, ?)",
            )?
            .execute([key, value])?;
        Ok(())
    }
}

macro_rules! config_impl {
    ($r_type:ty, $set:ident, $get:ident, $keys:ty) => {
        impl Database {
            pub fn $get(&self, key: $keys) -> $r_type {
                self.get_serde((&key).into())
                    .map(|e| serde_json::from_value(e).unwrap())
                    .unwrap_or(key.default_value())
            }
            pub fn $set(&self, key: $keys, value: $r_type) -> Result<()> {
                Ok(self.set_serde((&key).into(), &serde_json::to_string(&value)?)?)
            }
        }
    };
}

config_impl!(bool, set_bool, get_bool, BooleanKeys);
config_impl!(i64, set_integer, get_integer, IntegerKeys);
config_impl!(String, set_string, get_string, StringKeys);
