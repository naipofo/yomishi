use std::vec;

use yomishi_config::StringKeys::{AnkiConnectAddress, AnkiDeckName, AnkiModelName, AnkiTag};
use yomishi_proto::yomishi::anki::{
    OpenCardReply, OpenCardRequest, SaveDefinitionReply, SaveDefinitionRequest,
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
    fn save_definition(
        &self,
        SaveDefinitionRequest { scanned, index }: SaveDefinitionRequest,
    ) -> SaveDefinitionReply {
        self.runtime
            .block_on(self.add_to_anki(&search_to_template_data(
                self.search(&scanned).unwrap().remove(index as usize),
            )));
        SaveDefinitionReply {}
    }

    fn open_card(&self, OpenCardRequest { c_id }: OpenCardRequest) -> OpenCardReply {
        self.runtime
            .block_on(
                AnkiConnectClient::new(&self.storage.get_string(AnkiConnectAddress)).invoke(
                    &GuiBrowse {
                        query: &format!("cid:{c_id}"),
                    },
                ),
            )
            .unwrap();
        OpenCardReply {}
    }
}
impl Backend {
    async fn add_to_anki(&self, data: &GlossaryTemplateData) {
        let fields = self.render_anki_fields(data);
        let tag = self.storage.get_string(AnkiTag);
        let note_model = Note {
            deck_name: &self.storage.get_string(AnkiDeckName),
            model_name: &self.storage.get_string(AnkiModelName),
            fields: &fields.iter().map(|(a, b)| (a.as_str(), b.trim())).collect(),
            tags: &vec![&tag],
        };

        AnkiConnectClient::new(&self.storage.get_string(AnkiConnectAddress))
            .invoke(&AddNote { note: &note_model })
            .await
            .unwrap();
    }
}
