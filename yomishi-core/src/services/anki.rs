use crate::{
    anki_connect::{AddNote, AnkiConnectClient, CanAddNotes, Note},
    database::Database,
    html::{search_to_template_data, GlossaryTemplateData, HandlebarsRenderer},
    protos::yomishi::{
        anki::{self, SaveDefinitionReply, SaveDefinitionRequest},
        config::AnkiConnectConfig,
    },
};
use std::{collections::HashMap, sync::Arc};
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
    let deck_name = "test1";
    let model_name = "Novelcards"; // Model from my collection

    let client = AnkiConnectClient::new(&config.addrees);

    let hb = HandlebarsRenderer::new();

    let sample_conf = sample_conf();
    let fields = sample_conf
        .iter()
        .map(|(field, marker)| (field, hb.render_marker(&marker, data)))
        .collect::<Vec<_>>();
    let note_model = Note {
        deck_name,
        model_name,
        fields: &fields.iter().map(|(a, b)| (a.as_str(), b.trim())).collect(),
    };

    let can = client
        .can_add_notes(&CanAddNotes {
            notes: &vec![&note_model],
        })
        .await;

    println!("can ? {:?}", can);

    println!(
        "will ? {:?}",
        client.add_note(&AddNote { note: &note_model }).await
    );
}

fn sample_conf() -> HashMap<String, String> {
    let mut conf = HashMap::new();
    conf.insert("Word".to_string(), "ruby-plain".to_string());
    conf.insert("Glossary".to_string(), "glossary-list".to_string());
    conf
}
