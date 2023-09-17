use crate::{
    anki_connect::AnkiConnectClient,
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
    let deck = "test1";
    let model = "Novelcards"; // Model from my collection

    let client = AnkiConnectClient::new(&config.addrees);

    let hb = HandlebarsRenderer::new();

    client
        .add_note(
            &deck,
            &model,
            &sample_conf()
                .into_iter()
                .map(|(field, marker)| (field, hb.render_marker(&marker, data)))
                .collect(),
        )
        .await;
}

fn sample_conf() -> HashMap<String, String> {
    let mut conf = HashMap::new();
    conf.insert("Word".to_string(), "Expression".to_string());
    conf.insert("Reading".to_string(), "Reading".to_string());
    conf.insert("Furigana".to_string(), "RubyPlain".to_string());
    conf.insert("Glossary".to_string(), "GlossaryList".to_string());
    conf
}
