mod generator;
mod loader;

use std::{collections::HashMap, fs, path::Path};

use loader::load_toml;
use serde::Deserialize;
use strum::IntoStaticStr;

use crate::generator::generate_source;

pub fn compile_config_to_file(toml_def: &str, path: &str) {
    let text = generate_source(load_toml(toml_def).unwrap());
    let path = Path::new(path).join("config_gen.rs");
    println!("{:?}", path.canonicalize());
    fs::write(path, text).unwrap();
}

type ConfigData = HashMap<ConfigType, Vec<ConfigEntry>>;

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
