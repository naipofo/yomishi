use std::collections::HashMap;

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

async fn invoke<I: Serialize, O: DeserializeOwned>(action: &str, params: I) -> reqwest::Result<O> {
    let mut result = Client::new()
        .post("http://127.0.0.1:8765")
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

macro_rules! r {
    ($name:ident, $action:expr, $o:ty) => {
        pub async fn $name() -> $o {
            invoke($action, serde_json::json!({})).await.unwrap()
        }
    };
    ($name:ident, $action:expr, $i:ty, $o:ty) => {
        pub async fn $name(e: $i) -> $o {
            invoke($action, e).await.unwrap()
        }
    };
}

r!(deck_names, "deckNames", Vec<String>);
r!(model_names, "modelNames", Vec<String>);

pub async fn model_field_names(name: &str) -> Vec<String> {
    invoke("modelFieldNames", serde_json::json!({ "modelName": name }))
        .await
        .unwrap()
}

pub async fn find_notes(query: &str) -> Vec<i64> {
    invoke("findNotes", serde_json::json!({ "query": query }))
        .await
        .unwrap()
}

pub async fn add_note(deck: &str, model: &str, fields: &HashMap<String, String>) -> i64 {
    invoke(
        "addNote",
        serde_json::json!({ "note": { "deckName": deck, "modelName": model, "fields": fields }}),
    )
    .await
    .unwrap()
}
