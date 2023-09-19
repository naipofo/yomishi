use crate::{
    anki_connect::{AnkiConnectClient, CanAddNotes, Note},
    database::Database,
    flashcard::build_fields,
    html::{search_to_template_data, GlossaryTemplateData, HandlebarsRenderer},
    protos::yomishi::{
        config::AnkiConnectConfig,
        scan::{self, ScanResult, ScanStringReply, ScanStringRequest},
    },
};
use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use super::config::ConfigState;

pub struct ScanService {
    pub db: Arc<Mutex<Database>>,
    pub config: Arc<Mutex<ConfigState>>,
}

#[tonic::async_trait]
impl scan::scan_server::Scan for ScanService {
    async fn scan_string(
        &self,
        request: Request<ScanStringRequest>,
    ) -> Result<Response<ScanStringReply>, Status> {
        let config = &*self.config.lock().await;
        Ok(Response::new(ScanStringReply {
            results: join_all(
                self.db
                    .lock()
                    .await
                    .search(&request.get_ref().text)
                    .unwrap()
                    .into_iter()
                    .map(search_to_template_data)
                    .map(|e| data_to_result(e, &config.0.anki_connect.as_ref().unwrap())),
            )
            .await,
        }))
    }
}

async fn data_to_result(data: GlossaryTemplateData, config: &AnkiConnectConfig) -> ScanResult {
    let content = HandlebarsRenderer::new().render_glossary(&data);

    let client = AnkiConnectClient::new(&config.addrees);

    let fields = build_fields(&data, &config.fields);
    let note_model = Note {
        deck_name: &config.deck_name,
        model_name: &config.model_name,
        fields: &fields.iter().map(|(a, b)| (*a, b.trim())).collect(),
    };

    ScanResult {
        content,
        anki_can_add: (&mut client
            .can_add_notes(&CanAddNotes {
                notes: &vec![&note_model],
            })
            .await)
            .remove(0),
        card_id: None,
    }
}
