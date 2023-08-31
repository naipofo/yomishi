use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StructuredContent {
    Text(String),
    Multiple(Vec<StructuredItem>),
    Content(Box<StructuredItem>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StructuredItem {
    Text(String),
    Object {
        tag: String,
        #[serde(flatten)]
        data: ItemData,
        #[serde(flatten)]
        variant: Option<ItemVariant>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ItemVariant {
    Image {
        path: String,
    },
    Link {
        href: String,
        content: Option<StructuredContent>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemData {
    content: Option<StructuredContent>,
    style: Option<HashMap<String, Value>>,
    data: Option<HashMap<String, String>>,
}
