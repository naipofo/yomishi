mod generator;
mod generator_ts;
mod loader;

use std::{collections::HashMap, fs, path::Path};

use generator_ts::generate_source_ts;
use loader::load_toml;
use serde::Deserialize;
use strum::IntoStaticStr;

use crate::generator::generate_source;

pub fn compile_config_to_file(toml_def: &str, path: &str) {
    let text = generate_source(load_toml(toml_def).unwrap());
    let path = Path::new(path).join("config_gen.rs");
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

pub fn compile_ts_config_to_stdout(toml_def: &str) {
    let text = generate_source_ts(load_toml(toml_def).unwrap());
    print!("{text}");
}

// TODO: better structure for config / proto generators
// the structure of both is a very ugly and sometimes buggy
