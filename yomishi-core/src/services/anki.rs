use crate::{
    anki_connect::{AddNote, AnkiConnectClient, Note},
    database::Database,
    flashcard::build_fields,
    html::{search_to_template_data, GlossaryTemplateData},
    protos::yomishi::{
        anki::{self, SaveDefinitionReply, SaveDefinitionRequest},
        config::AnkiConnectConfig,
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use super::config::ConfigState;

pub struct AnkiService {
    pub db: Arc<Mutex<Database>>,
    pub config: Arc<Mutex<ConfigState>>,
}

#[tonic::async_trait]
impl anki::anki_server::Anki for AnkiService {
    async fn save_definition(
        &self,
        request: Request<SaveDefinitionRequest>,
    ) -> Result<Response<SaveDefinitionReply>, Status> {
        let SaveDefinitionRequest { scanned, index } = &request.get_ref();
        let data = search_to_template_data(
            self.db
                .lock()
                .await
                .search(&scanned)
                .unwrap()
                .remove(*index as usize),
        );
        let config = &*self.config.lock().await;

        add_to_anki(&data, &config.0.anki_connect.as_ref().unwrap()).await;

        Ok(Response::new(SaveDefinitionReply {}))
    }
}

async fn add_to_anki(data: &GlossaryTemplateData, config: &AnkiConnectConfig) {
    let client = AnkiConnectClient::new(&config.addrees);
    let fields = build_fields(data, &config.fields);
    let note_model = Note {
        deck_name: &config.deck_name,
        model_name: &config.model_name,
        fields: &fields.iter().map(|(a, b)| (*a, b.trim())).collect(),
    };

    client.add_note(&AddNote { note: &note_model }).await;
}
