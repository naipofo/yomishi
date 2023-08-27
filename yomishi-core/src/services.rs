use crate::{
    database::{slow_inmem::SlowInMemeoryDatabase, Database},
    protos::yomishi::scan::{self, ScanResult, ScanStringReply, ScanStringRequest},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub struct ScanService {
    pub db: Arc<Mutex<SlowInMemeoryDatabase>>,
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
                .into_iter()
                .map(|e| ScanResult {
                    expression: e.0.expression,
                    reading: e.0.reading,
                    inflection_rules: e.1.reasons.iter().map(|e| e.to_string()).collect(),
                    glossary: e.0.glossary,
                })
                .collect(),
        }))
    }
}
