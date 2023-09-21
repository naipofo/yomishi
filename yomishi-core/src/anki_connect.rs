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
        let mut result = self
            .client
            .post(self.address)
            .json(&serde_json::json!({
                "version": 6,
                "action": T::action(),
                "params": params,
            }))
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?;
        Ok(serde_json::from_value(result.remove("result").unwrap()).unwrap())
    }
}
