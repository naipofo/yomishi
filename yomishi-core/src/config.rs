use std::str::FromStr;
use std::vec;

use serde_json::Value;
use yomishi_config::StringKeys::{AnkiConnectAddress, AnkiModelName};
use yomishi_config::{BooleanKeys, IntegerKeys, SerdeKeys, StringKeys};
use yomishi_proto::yomishi::config::{
    AnkiConfigDataReply, AnkiConfigDataRequest, Config, ConfigType, Dictionary,
    DictionaryListReply, DictionaryListRequest, FetchConfigReply, FetchConfigRequest,
    PushConfigReply, PushConfigRequest,
};

use crate::{
    anki::connect::{
        actions::{DeckNames, ModelFieldNames, ModelNames},
        AnkiConnectClient,
    },
    backend::Backend,
    dict::DictIndex,
};

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

    fn dictionary_list(&self, _: DictionaryListRequest) -> DictionaryListReply {
        DictionaryListReply {
            dictionaries: self
                .storage
                .get_dicts()
                .unwrap()
                .into_iter()
                .map(
                    |(
                        id,
                        DictIndex {
                            title, revision, ..
                        },
                    )| Dictionary {
                        id,
                        name: format!("{} / {}", title, revision),
                    },
                )
                .collect(),
        }
    }

    fn anki_config_data(&self, _: AnkiConfigDataRequest) -> AnkiConfigDataReply {
        self.runtime.block_on(async {
            let address = self.storage.get_string(AnkiConnectAddress);
            let client = &AnkiConnectClient::new(&address);
            AnkiConfigDataReply {
                decks: client.invoke(&DeckNames {}).await.unwrap(),
                models: client.invoke(&ModelNames {}).await.unwrap(),
                current_model_fields: client
                    .invoke(&ModelFieldNames {
                        model_name: &self.storage.get_string(AnkiModelName),
                    })
                    .await
                    .unwrap_or(vec![]),
            }
        })
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
