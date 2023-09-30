use std::str::FromStr;

use serde_json::Value;
use yomishi_config::{BooleanKeys, IntegerKeys, StringKeys};
use yomishi_proto::yomishi::config::{Config, ConfigType, FetchConfigReply, FetchConfigRequest};

use crate::backend::Backend;

impl Config for Backend {
    fn fetch_config(&self, data: FetchConfigRequest) -> FetchConfigReply {
        FetchConfigReply {
            config: self.get_serialized(data),
        }
    }
}
impl Backend {
    fn get_serialized(&self, data: FetchConfigRequest) -> String {
        serde_json::to_string(&match ConfigType::try_from(data.r#type).unwrap() {
            ConfigType::Boolean => Value::Bool(
                self.storage
                    .get_bool(BooleanKeys::from_str(&data.key).unwrap()),
            ),
            ConfigType::Integer => Value::Number(
                self.storage
                    .get_integer(IntegerKeys::from_str(&data.key).unwrap())
                    .into(),
            ),
            ConfigType::String => Value::String(
                self.storage
                    .get_string(StringKeys::from_str(&data.key).unwrap()),
            ),
            // TODO: other types
            _ => panic!(),
        })
        .unwrap()
    }
}