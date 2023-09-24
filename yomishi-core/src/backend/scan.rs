use crate::{
    anki_connect::{
        actions::{CanAddNotes, FindNotes, Note},
        AnkiConnectClient,
    },
    error::Result,
    flashcard::build_fields,
    html::{search_to_template_data, GlossaryTemplateData, HandlebarsRenderer},
    protos::yomishi::{
        config::Config,
        scan::{self, ScanResult, ScanStringReply, ScanStringRequest},
    },
};
use futures::future::join_all;

use tonic::{Request, Response, Status};

use super::Backend;

#[tonic::async_trait]
impl scan::scan_server::Scan for Backend {
    async fn scan_string(
        &self,
        request: Request<ScanStringRequest>,
    ) -> Result<Response<ScanStringReply>, Status> {
        let config = self
            .with_dict(|dict| dict.storage.get_config())
            .await
            .unwrap();

        Ok(Response::new(ScanStringReply {
            results: join_all(
                self.with_dict(|dict| dict.search(&request.get_ref().text).unwrap())
                    .await
                    .into_iter()
                    .map(search_to_template_data)
                    .map(|e| data_to_result(e, &config)),
            )
            .await
            .into_iter()
            .collect::<Result<_>>()
            .unwrap(),
        }))
    }
}

async fn data_to_result(data: GlossaryTemplateData, config: &Config) -> Result<ScanResult> {
    let content = HandlebarsRenderer::new().render_glossary(&data);

    let client = AnkiConnectClient::new(&config.anki_connect_addrees);

    let fields = build_fields(&data, &config.anki_fields);
    let note_model = Note {
        deck_name: &config.anki_deck_name,
        model_name: &config.anki_model_name,
        fields: &fields.iter().map(|(a, b)| (*a, b.trim())).collect(),
    };

    let anki_can_add = client
        .invoke(&CanAddNotes {
            notes: &vec![&note_model],
        })
        .await?
        .remove(0);

    let card_id = Some(
        client
            .invoke(&FindNotes {
                query: &format!(
                    "Expression:{}",
                    data.glossaries.get(0).unwrap().data.term.expression
                ),
            })
            .await?
            .remove(0),
    );

    Ok(ScanResult {
        content,
        anki_can_add,
        card_id,
    })
}
