use crate::{
    database::{Database, SearchResult},
    dict::{html::HandlebarsRenderer, parser::term_meta::TermMetaEntry},
    japanese::ruby::try_from_reading,
    protos::yomishi::scan::{
        self, Frequency, Glossary, RubySegment, ScanResult, ScanStringReply, ScanStringRequest, Tag,
    },
};
use std::{collections::HashSet, sync::Arc};
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
        meta,
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
        frequency: meta
            .into_iter()
            .filter_map(|(dict, e)| match e.entry {
                TermMetaEntry::Frequency(s, n) => Some(Frequency {
                    name: dict,
                    value: match s {
                        Some(s) => s,
                        None => match n {
                            Some(n) => n.to_string(),
                            None => None?,
                        },
                    },
                }),
                TermMetaEntry::Pitches(_) => None, // TODO: Pitches
            })
            .collect(),
        glossary: glossares.into_iter().map(glossary_to_proto).collect(),
    }
}

fn glossary_to_proto(
    (term, tags): (
        crate::dict::parser::term::Term,
        Vec<crate::dict::parser::tag::Tag>,
    ),
) -> Glossary {
    Glossary {
        dictionary: "".to_string(),
        tags: tags.into_iter().map(tag_to_proto).collect(),
        definition: term
            .glossary
            .into_iter()
            .map(|e| HandlebarsRenderer::new().render_singular_glossary(&e))
            .collect(),
    }
}

fn tag_to_proto(e: crate::dict::parser::tag::Tag) -> Tag {
    Tag {
        name: e.name,
        description: e.notes,
        category: e.category,
    }
}
