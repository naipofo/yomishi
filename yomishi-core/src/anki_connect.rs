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
        impl AnkiConnectClient<'_> {
            pub async fn $name(&self) -> $o {
                self.invoke($action, serde_json::json!({})).await.unwrap()
            }
        }
    };
    ($name:ident, $action:expr, $i:ty, $o:ty) => {
        impl AnkiConnectClient<'_> {
            pub async fn $name(&self, e: &$i) -> $o {
                self.invoke($action, e).await.unwrap()
            }
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
        Ok(serde_json::from_value(result.remove("result").unwrap()).unwrap())
    }
}

r!(deck_names, "deckNames", Vec<String>);
r!(model_names, "modelNames", Vec<String>);

#[derive(Serialize)]
pub struct NotesQuery<'a> {
    pub query: &'a str,
}
r!(find_notes, "findNotes", NotesQuery<'_>, Vec<i64>);
r!(gui_browse, "guiBrowse", NotesQuery<'_>, Vec<i64>);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelNames<'a> {
    pub model_name: &'a str,
}
r!(
    model_field_names,
    "modelFieldNames",
    ModelNames<'_>,
    Vec<String>
);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note<'a> {
    pub deck_name: &'a str,
    pub model_name: &'a str,
    pub fields: &'a HashMap<&'a str, &'a str>,
}

#[derive(Serialize)]
pub struct AddNote<'a> {
    pub note: &'a Note<'a>,
}
r!(add_note, "addNote", AddNote<'_>, i64);

#[derive(Serialize)]
pub struct CanAddNotes<'a> {
    pub notes: &'a Vec<&'a Note<'a>>,
}
r!(can_add_notes, "canAddNotes", CanAddNotes<'_>, Vec<bool>);
