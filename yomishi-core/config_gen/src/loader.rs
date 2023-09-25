use std::collections::HashMap;

use serde::Deserialize;

use crate::{ConfigEntry, ConfigType};

#[derive(Debug, Deserialize)]
struct TypedEntry {
    r#type: ConfigType,
    #[serde(flatten)]
    entry: ConfigEntry,
}

pub fn load_toml(toml_def: &str) -> Result<HashMap<ConfigType, Vec<ConfigEntry>>, toml::de::Error> {
    let data: HashMap<String, HashMap<String, TypedEntry>> = toml::from_str(toml_def)?;
    let mut new_map = HashMap::new();

    for (category, category_map) in data.into_iter() {
        for (name, TypedEntry { r#type, entry }) in category_map {
            let vec = new_map.entry(r#type).or_insert_with(Vec::new);
            let name = format!("{}{}", category, name);
            vec.push(ConfigEntry { name, ..entry });
        }
    }

    Ok(new_map)
}
