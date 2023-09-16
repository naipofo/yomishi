use crate::{
    database::Database,
    protos::yomishi::anki::{self, SaveDefinitionReply, SaveDefinitionRequest},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub struct AnkiService {
    pub db: Arc<Mutex<Database>>,
}

#[tonic::async_trait]
impl anki::anki_server::Anki for AnkiService {
    async fn save_definition(
        &self,
        request: Request<SaveDefinitionRequest>,
    ) -> Result<Response<SaveDefinitionReply>, Status> {
        todo!()
    }
}
