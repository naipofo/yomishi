use crate::{ConfigData, ConfigEntry, ConfigType, SerdeTypes};

pub fn generate_source_ts(data: ConfigData) -> String {
    let mut buf = String::new();

    buf.push_str(r#"
    import { JsonValue } from "@bufbuild/protobuf";
    import { CONFIG_TYPE } from "../ts-protos/config_pb";
    
    export type ConfigInterfaceSpec<V extends JsonValue, Keys extends readonly string[], N extends string> = {
        name: N;
        type: CONFIG_TYPE;
        defaultValues: {
            [K in Keys[number]]: V;
        };
    };"#);

    for (config_type, entries) in &data {
        let lowercase_name = <&ConfigType as Into<&'static str>>::into(config_type).to_lowercase();
        let enum_name = format!("{}Keys", lowercase_name);

        buf.push_str(&format!("export const {enum_name} = [",));
        for key in entries {
            buf.push_str(&format!("\"{}\",", key.name));
        }
        buf.push_str("] as const;\n");

        buf.push_str(&format!(
            r#"
        export const {lowercase_name}InterfaceConfig: ConfigInterfaceSpec<{ts},typeof {enum_name}, "{ts}"> = {{
            name: "{ts}",
            type: CONFIG_TYPE.{prot},
            defaultValues: {{"#,
            ts = config_type.ts_type(),
            prot = config_type.proto_enum_name()
        ));
        for ConfigEntry { name, default, .. } in entries {
            buf.push_str(&format!(
                "\"{name}\": {},",
                config_type.ts_value_build(default)
            ));
        }
        buf.push_str("}} as const;\n");
        if let ConfigType::Serde = config_type {
            buf.push_str("export type serdeType<K extends typeof serdeKeys[number]> = ");
            for ConfigEntry { name, types, .. } in entries {
                if let Some(SerdeTypes { ts_type, .. }) = types {
                    buf.push_str(&format!("K extends \"{name}\" ? {ts_type} :",));
                }
            }
            buf.push_str("never;\n");
        }
    }

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
