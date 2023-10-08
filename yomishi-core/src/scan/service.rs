use std::collections::HashMap;

use yomishi_config::{
    SerdeKeys::AnkiFields,
    StringKeys::{AnkiConnectAddress, AnkiDeckName, AnkiModelName},
};
use yomishi_proto::yomishi::scan::{Scan, ScanResult, ScanStringReply, ScanStringRequest};

use crate::{
    anki::connect::{
        actions::{CanAddNotes, FindNotes, Note},
        AnkiConnectClient,
    },
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
        // TODO: defer ankiconnect calls to reduce scan time
        let fields = self.render_anki_fields(&data);
        let note_model = Note {
            deck_name: &self.storage.get_string(AnkiDeckName),
            model_name: &self.storage.get_string(AnkiModelName),
            fields: &fields.iter().map(|(a, b)| (a.as_str(), b.trim())).collect(),
            tags: &vec![],
        };

        self.runtime.block_on(async {
            Ok(ScanResult {
                content,
                anki_can_add: AnkiConnectClient::new(&self.storage.get_string(AnkiConnectAddress))
                    .invoke(&CanAddNotes {
                        notes: &vec![&note_model],
                    })
                    .await
                    .unwrap()
                    .remove(0),
                card_id: AnkiConnectClient::new(&self.storage.get_string(AnkiConnectAddress))
                    .invoke(&FindNotes {
                        query: &format!(
                            "{}:{}",
                            serde_json::from_value::<HashMap<String, String>>(
                                self.storage.get_serde(AnkiFields)
                            )?
                            .into_iter()
                            .find(|(_, value)| value == "Expression")
                            .map(|(key, _)| key)
                            .unwrap_or("Expression".to_string()),
                            HandlebarsRenderer::new()
                                .render_marker("expression", data)
                                .trim()
                        ),
                    })
                    .await
                    .unwrap()
                    .first()
                    .copied(),
            })
        })
    }
}
