pub mod actions;

use std::collections::HashMap;

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub struct AnkiConnectClient<'a> {
    address: &'a str,
    client: Client,
}

pub trait ConnectAction: Serialize {
    type Output: DeserializeOwned;
    fn action() -> &'static str;
}

impl AnkiConnectClient<'_> {
    pub fn new(address: &str) -> AnkiConnectClient {
        AnkiConnectClient {
            address,
            client: Client::new(),
        }
    }

    pub async fn invoke<T: ConnectAction>(&self, params: &T) -> reqwest::Result<T::Output> {
        self.invoke_any(params, T::action())
            .await
            .map(|e| serde_json::from_value(e).unwrap())
    }

    pub async fn invoke_any<T: Serialize>(&self, data: &T, action: &str) -> reqwest::Result<Value> {
        let mut result = self
            .client
            .post(self.address)
            .json(&serde_json::json!({
                "version": 6,
                "action":action,
                "params": data,
            }))
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?;
        Ok(serde_json::from_value(result.remove("result").unwrap()).unwrap())
    }
}
