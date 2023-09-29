use yomishi_proto::yomishi::scan::{Scan, ScanResult, ScanStringReply, ScanStringRequest};

use crate::{
    backend::Backend,
    error::Result,
    html::{search_to_template_data, GlossaryTemplateData, HandlebarsRenderer},
};

impl Scan for Backend {
    fn scan_string(&self, ScanStringRequest { text }: ScanStringRequest) -> ScanStringReply {
        ScanStringReply {
            results: self
                .search(&text)
                .unwrap()
                .into_iter()
                .map(search_to_template_data)
                .map(|e| self.data_to_result(e))
                .collect::<Result<_>>()
                .unwrap(),
        }
    }
}

impl Backend {
    fn data_to_result(&self, data: GlossaryTemplateData) -> Result<ScanResult> {
        let content = HandlebarsRenderer::new().render_glossary(&data);
        // TODO: anki integration
        // maybe defer it to reduce scan time?
        Ok(ScanResult {
            content,
            anki_can_add: false,
            card_id: Some(0),
        })
    }
}
