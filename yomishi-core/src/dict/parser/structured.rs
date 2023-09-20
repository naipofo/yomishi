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
#[serde(untagged, rename_all = "camelCase")]
pub enum ItemVariant {
    Image {
        path: String,
        width: Option<i32>,
        height: Option<i32>,
        title: Option<String>,
        size_units: Option<SizeUnits>,
    },
    Link {
        href: String,
    },
    TableElement {
        col_span: Option<i64>,
        row_span: Option<i64>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SizeUnits {
    Px,
    Em,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemData {
    pub content: Option<StructuredContent>,
    pub style: Option<HashMap<String, Value>>,
    pub data: Option<HashMap<String, String>>,
}
