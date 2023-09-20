use std::collections::HashMap;

use crate::html::{GlossaryTemplateData, HandlebarsRenderer};

pub fn build_fields<'a>(
    data: &GlossaryTemplateData,
    config: &'a HashMap<String, String>,
) -> Vec<(&'a str, String)> {
    let hb = HandlebarsRenderer::new();
    config
        .into_iter()
        .map(|(field, marker)| (field.as_str(), hb.render_marker(&marker, data)))
        .collect::<Vec<_>>()
}
