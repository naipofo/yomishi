use std::collections::HashMap;

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub struct AnkiConnectClient<'a> {
    address: &'a str,
    client: Client,
}

macro_rules! r {
    ($name:ident, $action:expr, $o:ty) => {
        pub async fn $name(&self) -> $o {
            self.invoke($action, serde_json::json!({})).await.unwrap()
        }
    };
    ($name:ident, $action:expr, $i:ty, $o:ty) => {
        pub async fn $name(&self, e: $i) -> $o {
            self.invoke($action, e).await.unwrap()
        }
    };
}

impl AnkiConnectClient<'_> {
    pub fn new(address: &str) -> AnkiConnectClient {
        AnkiConnectClient {
            address,
            client: Client::new(),
        }
    }

    async fn invoke<I: Serialize, O: DeserializeOwned>(
        &self,
        action: &str,
        params: I,
    ) -> reqwest::Result<O> {
        let mut result = self
            .client
            .post(self.address)
            .json(&serde_json::json!({
                "version": 6,
                "action": action,
                "params": params,
            }))
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?;
        println!("{:?}", result);
        Ok(serde_json::from_value(result.remove("result").unwrap()).unwrap())
    }
    r!(deck_names, "deckNames", Vec<String>);
    r!(model_names, "modelNames", Vec<String>);

    pub async fn model_field_names(&self, name: &str) -> Vec<String> {
        self.invoke("modelFieldNames", serde_json::json!({ "modelName": name }))
            .await
            .unwrap()
    }

    pub async fn find_notes(&self, query: &str) -> Vec<i64> {
        self.invoke("findNotes", serde_json::json!({ "query": query }))
            .await
            .unwrap()
    }

    pub async fn add_note(&self, deck: &str, model: &str, fields: &HashMap<String, String>) -> i64 {
        self.invoke(
            "addNote",
            serde_json::json!({ "note": { "deckName": deck, "modelName": model, "fields": fields }}),
        )
        .await
        .unwrap()
    }
}
