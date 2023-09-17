use crate::{
    database::{Database, SearchResult},
    dict::html::{GlossaryTemplateData, HandlebarsRenderer},
    japanese::ruby::try_from_reading,
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
                .map(search_to_proto)
                .map(|data| HandlebarsRenderer::new().render_glossary(data))
                .collect(),
        }))
    }
}

fn search_to_proto(result: SearchResult) -> GlossaryTemplateData {
    let SearchResult {
        deinflection,
        glossaries,
        tags,
        meta,
    } = result;

    GlossaryTemplateData {
        ruby: try_from_reading(
            glossaries.get(0).unwrap().term.expression.to_string(),
            glossaries.get(0).unwrap().term.reading.to_string(),
        ),
        inflection_rules: deinflection.reasons.iter().map(|e| e.to_string()).collect(),
        tags,
        meta,
        glossaries,
    }
}
