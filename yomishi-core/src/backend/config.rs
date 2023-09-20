use crate::protos::yomishi::config::{
    self, AnkiConnectConfig, Config, FetchConfigReply, FetchConfigRequest, PushConfigReply,
    PushConfigRequest,
};
use tonic::{Request, Response, Status};

use super::Backend;

#[tonic::async_trait]
impl config::config_service_server::ConfigService for Backend {
    async fn fetch_config(
        &self,
        _: Request<FetchConfigRequest>,
    ) -> Result<Response<FetchConfigReply>, Status> {
        Ok(Response::new(FetchConfigReply {
            config: Some(self.config.lock().await.0.clone()),
        }))
    }
    async fn push_config(
        &self,
        request: Request<PushConfigRequest>,
    ) -> Result<Response<PushConfigReply>, Status> {
        let res = &request.get_ref().config;
        let mut conf = self.config.lock().await;
        match res {
            Some(e) => conf.0 = e.clone(),
            None => todo!(),
        }
        Ok(Response::new(PushConfigReply {}))
    }
}

pub fn default_config() -> Config {
    Config {
        anki_connect: Some(AnkiConnectConfig {
            enabled: true,
            addrees: "http://127.0.0.1:8765".to_string(),
            // Model from my collection
            deck_name: "test1".to_string(),
            model_name: "Novelcards".to_string(),
            fields: vec![
                ("Word".to_string(), "ruby-plain".to_string()),
                ("Glossary".to_string(), "glossary-list".to_string()),
            ]
            .into_iter()
            .collect(),
        }),
    }
}
