use std::str::FromStr;

use serde_json::Value;
use yomishi_config::{BooleanKeys, IntegerKeys, SerdeKeys, StringKeys};
use yomishi_proto::yomishi::config::{
    Config, ConfigType, FetchConfigReply, FetchConfigRequest, PushConfigReply, PushConfigRequest,
};

use crate::backend::Backend;

impl Config for Backend {
    fn fetch_config(&self, data: FetchConfigRequest) -> FetchConfigReply {
        FetchConfigReply {
            config: self.get_serialized(data),
        }
    }

    fn push_config(
        &self,
        PushConfigRequest { r#type, key, value }: PushConfigRequest,
    ) -> PushConfigReply {
        match ConfigType::try_from(r#type).unwrap() {
            ConfigType::Boolean => self.storage.set_bool(
                BooleanKeys::from_str(&key).unwrap(),
                serde_json::from_str(&value).unwrap(),
            ),
            ConfigType::String => self.storage.set_string(
                StringKeys::from_str(&key).unwrap(),
                serde_json::from_str(&value).unwrap(),
            ),
            ConfigType::Integer => self.storage.set_integer(
                IntegerKeys::from_str(&key).unwrap(),
                serde_json::from_str(&value).unwrap(),
            ),
            ConfigType::Serde => self.storage.set_serde(
                SerdeKeys::from_str(&key).unwrap(),
                serde_json::from_str(&value).unwrap(),
            ),
        }
        .unwrap();

        PushConfigReply {}
    }
}

impl Backend {
    fn get_serialized(&self, FetchConfigRequest { r#type, key }: FetchConfigRequest) -> String {
        serde_json::to_string(&match ConfigType::try_from(r#type).unwrap() {
            ConfigType::Boolean => {
                Value::Bool(self.storage.get_bool(BooleanKeys::from_str(&key).unwrap()))
            }
            ConfigType::Integer => Value::Number(
                self.storage
                    .get_integer(IntegerKeys::from_str(&key).unwrap())
                    .into(),
            ),
            ConfigType::String => {
                Value::String(self.storage.get_string(StringKeys::from_str(&key).unwrap()))
            }
            ConfigType::Serde => self.storage.get_serde(SerdeKeys::from_str(&key).unwrap()),
        })
        .unwrap()
    }
}
