use crate::{
    database::{slow_inmem::SlowInMemeoryDatabase, Database, SearchResult},
    japanese::ruby::try_from_reading,
    protos::yomishi::scan::{self, RubySegment, ScanResult, ScanStringReply, ScanStringRequest},
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
                .map(search_to_proto)
                .collect(),
        }))
    }
}

fn search_to_proto(e: SearchResult) -> ScanResult {
    ScanResult {
        ruby: try_from_reading(e.0.expression, e.0.reading)
            .into_iter()
            .map(|e| match e {
                crate::japanese::ruby::Segment::Text(text) => RubySegment { text, ruby: None },
                crate::japanese::ruby::Segment::Ruby(text, r) => RubySegment {
                    text,
                    ruby: Some(r),
                },
            })
            .collect(),
        inflection_rules: e.1.reasons.iter().map(|e| e.to_string()).collect(),
        glossary: e.0.glossary,
    }
}
