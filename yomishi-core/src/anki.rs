use std::collections::HashMap;

use crate::{
    backend::Backend,
    html::{GlossaryTemplateData, HandlebarsRenderer},
};

pub mod connect;
mod service;

impl Backend {
    pub fn render_anki_fields(&self, data: &GlossaryTemplateData) -> Vec<(String, String)> {
        let hb = HandlebarsRenderer::new();
        let config: HashMap<String, String> = serde_json::from_value(
            self.storage
                .get_serde(yomishi_config::SerdeKeys::AnkiFields),
        )
        .unwrap();
        config
            .into_iter()
            .map(|(field, marker)| (field, hb.render_marker(&marker, data)))
            .collect::<Vec<_>>()
    }
}
