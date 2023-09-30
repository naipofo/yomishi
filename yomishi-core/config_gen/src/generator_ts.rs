use crate::{ConfigData, ConfigType};

pub fn generate_source_ts(data: ConfigData) -> String {
    let mut buf = String::new();

    for (config_type, entries) in data {
        let enum_name = format!(
            "{}_keys",
            <&ConfigType as Into<&'static str>>::into(&config_type).to_lowercase()
        );

        buf.push_str(&format!("export const {enum_name} = [",));
        for key in &entries {
            buf.push_str(&format!("\"{}\",", key.name));
        }
        buf.push_str("] as const;\n");
    }

    buf
}
