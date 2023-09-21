use crate::{
    anki_connect::{
        actions::{CanAddNotes, FindNotes, Note},
        AnkiConnectClient,
    },
    error::Result,
    flashcard::build_fields,
    html::{search_to_template_data, GlossaryTemplateData, HandlebarsRenderer},
    protos::yomishi::{
        config::AnkiConnectConfig,
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
        let config = &*self.config.lock().await;
        let anki_connect = config.0.anki_connect.clone().unwrap();

        Ok(Response::new(ScanStringReply {
            results: join_all(
                self.with_dict(|dict| dict.search(&request.get_ref().text).unwrap())
                    .await
                    .into_iter()
                    .map(search_to_template_data)
                    .map(|e| data_to_result(e, &anki_connect)),
            )
            .await
            .into_iter()
            .collect::<Result<_>>()
            .unwrap(),
        }))
    }
}

async fn data_to_result(
    data: GlossaryTemplateData,
    config: &AnkiConnectConfig,
) -> Result<ScanResult> {
    let content = HandlebarsRenderer::new().render_glossary(&data);

    let client = AnkiConnectClient::new(&config.addrees);

    let fields = build_fields(&data, &config.fields);
    let note_model = Note {
        deck_name: &config.deck_name,
        model_name: &config.model_name,
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
