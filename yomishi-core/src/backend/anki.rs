use crate::{
    anki_connect::{
        actions::{AddNote, GuiBrowse, Note},
        AnkiConnectClient,
    },
    flashcard::build_fields,
    html::{search_to_template_data, GlossaryTemplateData},
    protos::yomishi::{
        anki::{self, OpenCardReply, OpenCardRequest, SaveDefinitionReply, SaveDefinitionRequest},
        config::AnkiConnectConfig,
    },
};
use tonic::{Request, Response, Status};

use super::Backend;

#[tonic::async_trait]
impl anki::anki_server::Anki for Backend {
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

    async fn open_card(
        &self,
        request: Request<OpenCardRequest>,
    ) -> Result<Response<OpenCardReply>, Status> {
        let config = &*self.config.lock().await;
        let client = AnkiConnectClient::new(&config.0.anki_connect.as_ref().unwrap().addrees);

        client
            .invoke(&GuiBrowse {
                query: &format!("cid:{}", request.get_ref().c_id),
            })
            .await
            .unwrap();

        Ok(Response::new(OpenCardReply {}))
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

    client.invoke(&AddNote { note: &note_model }).await.unwrap();
}
