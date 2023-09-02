use crate::{
    database::Database,
    deinflector::DeinflectionMeta,
    dict::{html::render_glossary, parser::term::Term},
    japanese::ruby::try_from_reading,
    protos::yomishi::scan::{self, RubySegment, ScanResult, ScanStringReply, ScanStringRequest},
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
                .flat_map(search_to_proto)
                .collect(),
        }))
    }
}

fn search_to_proto(e: (DeinflectionMeta, Vec<Term>)) -> Vec<ScanResult> {
    let (meta, terms) = e;
    terms
        .into_iter()
        .map(|term| ScanResult {
            ruby: try_from_reading(term.expression, term.reading)
                .into_iter()
                .map(|e| match e {
                    crate::japanese::ruby::Segment::Text(text) => RubySegment { text, ruby: None },
                    crate::japanese::ruby::Segment::Ruby(text, r) => RubySegment {
                        text,
                        ruby: Some(r),
                    },
                })
                .collect(),
            inflection_rules: meta.reasons.iter().map(|e| e.to_string()).collect(),
            glossary: term
                .glossary
                .into_iter()
                .map(|e| render_glossary(e).unwrap().0)
                .collect(),
        })
        .collect()
}
