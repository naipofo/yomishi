use crate::{
    database::{Database, SearchResult},
    dict::html::render_glossary,
    japanese::ruby::try_from_reading,
    protos::yomishi::scan::{
        self, Glossary, RubySegment, ScanResult, ScanStringReply, ScanStringRequest, Tag,
    },
};
use std::{collections::HashSet, sync::Arc, vec};
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
                .collect(),
        }))
    }
}

fn search_to_proto(result: SearchResult) -> ScanResult {
    let SearchResult {
        deinflection,
        glossares,
        tags,
    } = result;

    ScanResult {
        ruby: try_from_reading(
            glossares.get(0).unwrap().0.expression.to_string(),
            glossares.get(0).unwrap().0.reading.to_string(),
        )
        .into_iter()
        .map(|e| match e {
            crate::japanese::ruby::Segment::Text(text) => RubySegment { text, ruby: None },
            crate::japanese::ruby::Segment::Ruby(text, r) => RubySegment {
                text,
                ruby: Some(r),
            },
        })
        .collect(),
        inflection_rules: deinflection.reasons.iter().map(|e| e.to_string()).collect(),
        tags: tags
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .map(tag_to_proto)
            .collect(),
        frequency: vec![],
        glossary: glossares.into_iter().map(glossary_to_proto).collect(),
    }
}

fn glossary_to_proto(
    g: (
        crate::dict::parser::term::Term,
        Vec<crate::dict::parser::term_meta::TermMeta>,
        Vec<crate::dict::parser::tag::Tag>,
    ),
) -> Glossary {
    Glossary {
        dictionary: "".to_string(),
        tags: g.2.into_iter().map(tag_to_proto).collect(),
        definition: g
            .0
            .glossary
            .into_iter()
            .map(|e| render_glossary(e).unwrap().0)
            .collect(),
    }
}

fn tag_to_proto(e: crate::dict::parser::tag::Tag) -> Tag {
    Tag {
        name: e.name,
        description: e.notes,
    }
}
