use crate::protos::yomishi::config::{
    self, AnkiConnectConfig, Config, FetchConfigReply, FetchConfigRequest, PushConfigReply,
    PushConfigRequest,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub struct ConfigState(pub Config);

pub struct ConfigService {
    pub config: Arc<Mutex<ConfigState>>,
}

#[tonic::async_trait]
impl config::config_service_server::ConfigService for ConfigService {
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
        }),
    }
}
