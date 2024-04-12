pub mod actions;

use std::collections::HashMap;

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::error::Result;

pub struct AnkiConnectClient<'a> {
    address: &'a str,
}

pub trait ConnectAction: Serialize {
    type Output: DeserializeOwned;
    fn action() -> &'static str;
}

impl AnkiConnectClient<'_> {
    pub fn new(address: &str) -> AnkiConnectClient {
        AnkiConnectClient { address }
    }

    pub async fn invoke<T: ConnectAction>(&self, params: &T) -> Result<T::Output> {
        Ok(serde_json::from_value(
            self.invoke_any(params, T::action()).await?,
        )?)
    }

    async fn invoke_any<T: Serialize>(&self, data: &T, action: &str) -> reqwest::Result<Value> {
        let mut result = Client::new()
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
