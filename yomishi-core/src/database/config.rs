use serde_json::Value;
use yomishi_config::{BooleanKeys, IntegerKeys, SerdeKeys, StringKeys};

use crate::error::Result;

use super::Database;

impl Database {
    // TODO: different result for db error / no value
    pub fn get_generic(&self, key: &str) -> Result<Value> {
        Ok(serde_json::from_str(
            &self
                .conn
                .prepare(
                    "SELECT value, id
                    FROM config
                    WHERE key = ?
					ORDER BY id DESC",
                )?
                .query_row([key], |row| row.get::<_, String>(0))?,
        )?)
    }

    pub fn set_generic(&self, key: &str, value: &str) -> Result<()> {
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
                self.get_generic((&key).into())
                    .map(|e| serde_json::from_value(e).unwrap())
                    .unwrap_or(key.default_value())
            }
            pub fn $set(&self, key: $keys, value: $r_type) -> Result<()> {
                self.set_generic((&key).into(), &serde_json::to_string(&value)?)
            }
        }
    };
}

config_impl!(bool, set_bool, get_bool, BooleanKeys);
config_impl!(i64, set_integer, get_integer, IntegerKeys);
config_impl!(String, set_string, get_string, StringKeys);
config_impl!(Value, set_serde, get_serde, SerdeKeys);
