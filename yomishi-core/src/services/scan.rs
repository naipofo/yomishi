use crate::{
    database::Database,
    html::{search_to_template_data, HandlebarsRenderer},
    protos::yomishi::scan::{self, ScanStringReply, ScanStringRequest},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub struct ScanService {
    pub db: Arc<Mutex<Database>>,
}

#[tonic::async_trait]
impl scan::scan_server::Scan for ScanService {
    async fn scan_string(
        &self,
        request: Request<ScanStringRequest>,
    ) -> Result<Response<ScanStringReply>, Status> {
        Ok(Response::new(ScanStringReply {
            results: self
                .db
                .lock()
                .await
                .search(&request.get_ref().text)
                .unwrap()
                .into_iter()
                .map(search_to_template_data)
                .map(|data| HandlebarsRenderer::new().render_glossary(data))
                .collect(),
        }))
    }
}
