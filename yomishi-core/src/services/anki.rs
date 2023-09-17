use crate::{
    anki_connect::AnkiConnectClient,
    database::Database,
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
        let res = &request.get_ref().result;
        let config = &*self.config.lock().await;
        // Ok(Response::new(SaveDefinitionReply {}));
        todo!()
    }
}

async fn add_to_anki(result: &str, config: &AnkiConnectConfig) {
    let deck = "test1";
    let model = "Novelcards"; // Model from my collection

    let client = AnkiConnectClient::new(&config.addrees);

    client.add_note(&deck, &model, &HashMap::new()).await;
}
