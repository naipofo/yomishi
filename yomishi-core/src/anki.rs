use std::collections::HashMap;

use yomishi_proto::yomishi::anki::ClientState;

use crate::{
    backend::Backend,
    html::{GlossaryTemplateData, HandlebarsRenderer},
};

pub mod connect;
mod service;

impl Backend {
    pub fn render_anki_fields(
        &self,
        data: &GlossaryTemplateData,
        state: &Option<ClientState>,
    ) -> Vec<(String, String)> {
        let hb = HandlebarsRenderer::new();
        let config: HashMap<String, String> = serde_json::from_value(
            self.storage
                .get_serde(yomishi_config::SerdeKeys::AnkiFields),
        )
        .unwrap();
        config
            .into_iter()
            .map(|(field, marker)| {
                (
                    field,
                    match marker.as_str() {
                        "clipboard" => state.as_ref().map(|s| s.clipboard.to_string()),
                        "selection" => state.as_ref().map(|s| s.selection.to_string()),
                        _ => Some(hb.render_marker(&marker, data)),
                    }
                    .unwrap_or(String::from("")),
                )
            })
            .collect::<Vec<_>>()
    }
}
