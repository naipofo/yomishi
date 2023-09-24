use crate::{
    anki_connect::{
        actions::{AddNote, GuiBrowse, Note},
        AnkiConnectClient,
    },
    flashcard::build_fields,
    html::{search_to_template_data, GlossaryTemplateData},
    protos::yomishi::{
        anki::{self, OpenCardReply, OpenCardRequest, SaveDefinitionReply, SaveDefinitionRequest},
        config::Config,
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
        let config = self
            .with_dict(|dict| dict.storage.get_config())
            .await
            .unwrap();
        let SaveDefinitionRequest { scanned, index } = &request.get_ref();
        let data = search_to_template_data(
            self.db
                .lock()
                .await
                .search(&scanned)
                .unwrap()
                .remove(*index as usize),
        );

        add_to_anki(&data, &config).await;

        Ok(Response::new(SaveDefinitionReply {}))
    }

    async fn open_card(
        &self,
        request: Request<OpenCardRequest>,
    ) -> Result<Response<OpenCardReply>, Status> {
        let config = self
            .with_dict(|dict| dict.storage.get_config().unwrap())
            .await;
        let client = AnkiConnectClient::new(&config.anki_connect_addrees);

        client
            .invoke(&GuiBrowse {
                query: &format!("cid:{}", request.get_ref().c_id),
            })
            .await
            .unwrap();

        Ok(Response::new(OpenCardReply {}))
    }
}

async fn add_to_anki(data: &GlossaryTemplateData, config: &Config) {
    let client = AnkiConnectClient::new(&config.anki_connect_addrees);
    let fields = build_fields(data, &config.anki_fields);
    let note_model = Note {
        deck_name: &config.anki_deck_name,
        model_name: &config.anki_model_name,
        fields: &fields.iter().map(|(a, b)| (*a, b.trim())).collect(),
    };

    client.invoke(&AddNote { note: &note_model }).await.unwrap();
}
