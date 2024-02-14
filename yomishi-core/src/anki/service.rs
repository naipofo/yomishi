use std::vec;

use yomishi_config::StringKeys::{AnkiConnectAddress, AnkiDeckName, AnkiModelName, AnkiTag};
use yomishi_proto::yomishi::anki::{
    ClientState, OpenCardReply, OpenCardRequest, SaveDefinitionReply, SaveDefinitionRequest,
};

use crate::{
    backend::Backend,
    html::{search_to_template_data, GlossaryTemplateData},
};

use super::connect::{
    actions::{AddNote, GuiBrowse, Note},
    AnkiConnectClient,
};

impl yomishi_proto::yomishi::anki::Anki for Backend {
    async fn save_definition(
        &self,
        SaveDefinitionRequest {
            scanned,
            index,
            state,
        }: SaveDefinitionRequest,
    ) -> SaveDefinitionReply {
        self.add_to_anki(
            &search_to_template_data(self.search(&scanned).await.unwrap().remove(index as usize)),
            &state,
        )
        .await;
        SaveDefinitionReply {}
    }

    async fn open_card(&self, OpenCardRequest { c_id }: OpenCardRequest) -> OpenCardReply {
        AnkiConnectClient::new(&self.storage.get_string(AnkiConnectAddress).await)
            .invoke(&GuiBrowse {
                query: &format!("cid:{c_id}"),
            })
            .await
            .unwrap();
        OpenCardReply {}
    }
}
impl Backend {
    async fn add_to_anki(&self, data: &GlossaryTemplateData, state: &Option<ClientState>) {
        let fields = self.render_anki_fields(data, state).await;
        let tag = self.storage.get_string(AnkiTag).await;
        let note_model = Note {
            deck_name: &self.storage.get_string(AnkiDeckName).await,
            model_name: &self.storage.get_string(AnkiModelName).await,
            fields: &fields.iter().map(|(a, b)| (a.as_str(), b.trim())).collect(),
            tags: &vec![&tag],
        };

        AnkiConnectClient::new(&self.storage.get_string(AnkiConnectAddress).await)
            .invoke(&AddNote { note: &note_model })
            .await
            .unwrap();
    }
}
