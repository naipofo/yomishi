use crate::protos::yomishi::config::{
    self, Config, FetchConfigReply, FetchConfigRequest, PushConfigReply, PushConfigRequest,
};
use tonic::{Request, Response, Status};

use super::Backend;

#[tonic::async_trait]
impl config::config_service_server::ConfigService for Backend {
    async fn fetch_config(
        &self,
        _: Request<FetchConfigRequest>,
    ) -> Result<Response<FetchConfigReply>, Status> {
        let config = Some(
            self.with_dict(|dict| dict.storage.get_config())
                .await
                .unwrap_or(default_config()),
        );
        Ok(Response::new(FetchConfigReply { config }))
    }
    async fn push_config(
        &self,
        request: Request<PushConfigRequest>,
    ) -> Result<Response<PushConfigReply>, Status> {
        let res = &request.get_ref().config;
        match res.as_ref() {
            Some(c) => {
                self.with_dict(|dict| dict.storage.set_config(c))
                    .await
                    .unwrap();
            }
            None => {}
        }
        Ok(Response::new(PushConfigReply {}))
    }
}

pub fn default_config() -> Config {
    Config {
        disabled_dictionaries: vec![],
        popup_width: 600,
        popup_height: 300,
        anki_enabled: true,
        anki_connect_addrees: "http://127.0.0.1:8765".to_string(),

        // Model from my collection
        anki_deck_name: "test1".to_string(),
        anki_model_name: "Novelcards".to_string(),
        anki_tag: "yomishi".to_string(),
        anki_fields: vec![
            ("Expression".to_string(), "ruby-plain".to_string()),
            ("Glossary".to_string(), "glossary-list".to_string()),
        ]
        .into_iter()
        .collect(),
    }
}
