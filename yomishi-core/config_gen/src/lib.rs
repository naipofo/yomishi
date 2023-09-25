mod loader;

use std::{fs, path::Path};

use loader::load_toml;
use serde::Deserialize;
use strum::IntoStaticStr;

pub fn compile_config_to_file(toml_def: &str, path: &str) {
    let text = compile_config(toml_def).unwrap();
    let path = Path::new(path).join("config_gen.rs");
    println!("{:?}", path.canonicalize());
    fs::write(path, text).unwrap();
}

fn compile_config(toml_def: &str) -> Result<String, toml::de::Error> {
    let data = load_toml(toml_def)?;
    let mut buf = String::new();

    for (config_type, entries) in data {
        buf.push_str(&format!(
            "#[derive(Debug, strum::IntoStaticStr)]\npub enum {}Keys {{\n",
            <ConfigType as Into<&'static str>>::into(config_type)
        ));

        for key in entries {
            buf.push_str(&format!("{},\n", key.name));
        }

        buf.push_str("}\n");
    }

    Ok(format!("{}", buf))
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, IntoStaticStr)]
pub enum ConfigType {
    String,
    Boolean,
    Integer,
    Serde,
}

#[derive(Debug, Deserialize)]
pub struct ConfigEntry {
    #[serde(default = "String::new")]
    name: String,
    default: String,
}
