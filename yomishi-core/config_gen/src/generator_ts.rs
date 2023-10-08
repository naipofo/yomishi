use crate::{ConfigData, ConfigType};

pub fn generate_source_ts(data: ConfigData) -> String {
    let mut buf = String::new();

    buf.push_str(
        r#"
    import { CONFIG_TYPE } from "../ts-protos/config_pb";

    export interface ConfigKey<T> {
        name: string;
        default: T;
    }
    export interface ProtoTyped{
        type: CONFIG_TYPE;
    }

    export const rpcKeys = {
    "#,
    );
    for (config_type, entries) in &data {
        for entry in entries {
            let ts_type = match &entry.types {
                Some(t) => t.ts_type.to_string(),
                None => config_type.ts_type().to_string(),
            };

            buf.push_str(&format!(
                "{name}:{{
                    name: \"{name}\" as const,
                    default: {default},
                    type: CONFIG_TYPE.{proto},
            }} as ConfigKey<{ts_type}> & ProtoTyped & {{ name: \"{name}\" }},
            ",
                name = entry.name, // TODO: first letter lowercase
                default = config_type.ts_value_build(&entry.default),
                proto = config_type.proto_enum_name()
            ));
        }
    }
    buf.push_str("} ;\n");
    buf
}

impl ConfigType {
    // TODO: get rid of this in favor of one more dot on client
    fn ts_type(&self) -> &'static str {
        match self {
            ConfigType::String => "string",
            ConfigType::Boolean => "boolean",
            ConfigType::Integer => "number",
            ConfigType::Serde => "any",
        }
    }
    fn ts_value_build(&self, value: &str) -> String {
        match self {
            ConfigType::String => format!("\"{}\"", value),
            _ => value.to_string(),
        }
    }
    fn proto_enum_name(&self) -> &'static str {
        match self {
            ConfigType::String => "STRING",
            ConfigType::Boolean => "BOOLEAN",
            ConfigType::Integer => "INTEGER",
            ConfigType::Serde => "SERDE",
        }
    }
}
