use crate::{ConfigData, ConfigType};

pub fn generate_source(data: ConfigData) -> String {
    let mut buf = String::new();

    for (config_type, entries) in data {
        let enum_name = format!(
            "{}Keys",
            <&ConfigType as Into<&'static str>>::into(&config_type)
        );

        buf.push_str(&format!(
            "#[derive(Debug, strum::IntoStaticStr)]\npub enum {enum_name} {{\n",
        ));
        for key in &entries {
            buf.push_str(&format!("{},\n", key.name));
        }
        buf.push_str("}\n");

        buf.push_str(&format!(
            "impl {enum_name}{{\nfn default_value(&self) ->{}{{match self {{",
            config_type.return_type(),
        ));
        for key in &entries {
            buf.push_str(&format!(
                "Self::{} => {},\n",
                key.name,
                config_type.value_build(&key.default)
            ));
        }

        buf.push_str("}}}\n");
    }

    buf
}

impl ConfigType {
    fn return_type(&self) -> &'static str {
        match self {
            ConfigType::String => "&'static str",
            ConfigType::Boolean => "bool",
            ConfigType::Integer => "i64",
            ConfigType::Serde => "serde_json::Value",
        }
    }

    fn value_build(&self, value: &str) -> String {
        match self {
            ConfigType::String => format!("\"{}\"", value),
            ConfigType::Boolean => value.to_string(),
            ConfigType::Integer => format!("{}i64", value),
            ConfigType::Serde => format!("serde_json::json!({})", value),
        }
    }
}
