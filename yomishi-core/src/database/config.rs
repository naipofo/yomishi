use serde::{Deserialize, Serialize};
use serde_json::Value;
use yomishi_config::{BooleanKeys, IntegerKeys, SerdeKeys, StringKeys};

use crate::error::{yo_er, Result};

use super::Database;

#[derive(Debug, Deserialize)]
struct SurConfig {
    value: String,
}

#[derive(Debug, Serialize)]
struct SurUpdateConfig<'a> {
    value: &'a str,
}

impl Database {
    // TODO: different result for db error / no value
    pub async fn get_generic(&self, key: &str) -> Result<Value> {
        let a: Option<SurConfig> = self
            .conn
            .select(("config", key))
            .await
            .map_err(|_| yo_er!())?;
        let value = a.ok_or(yo_er!())?;
        Ok(serde_json::from_str(&value.value).unwrap())
    }

    pub async fn set_generic(&self, key: &str, value: &str) -> Result<()> {
        let _: Option<SurConfig> = self
            .conn
            .update(("config", key))
            .content(SurUpdateConfig { value })
            .await
            .unwrap();
        Ok(())
    }
}

macro_rules! config_impl {
    ($r_type:ty, $set:ident, $get:ident, $keys:ty) => {
        impl Database {
            pub async fn $get(&self, key: $keys) -> $r_type {
                self.get_generic((&key).into())
                    .await
                    .map(|e| serde_json::from_value(e).unwrap())
                    .unwrap_or(key.default_value())
            }
            pub async fn $set(&self, key: $keys, value: $r_type) -> Result<()> {
                self.set_generic((&key).into(), &serde_json::to_string(&value)?)
                    .await
            }
        }
    };
}

config_impl!(bool, set_bool, get_bool, BooleanKeys);
config_impl!(i64, set_integer, get_integer, IntegerKeys);
config_impl!(String, set_string, get_string, StringKeys);
config_impl!(Value, set_serde, get_serde, SerdeKeys);
