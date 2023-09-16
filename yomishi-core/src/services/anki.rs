use crate::{
    anki_connect::add_note,
    database::Database,
    protos::yomishi::{
        anki::{self, SaveDefinitionReply, SaveDefinitionRequest},
        scan::{RubySegment, ScanResult},
    },
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub struct AnkiService {
    pub db: Arc<Mutex<Database>>,
}

enum Field {
    Expression,
    Reading,
    Furigana,
    Glossary,
}

#[tonic::async_trait]
impl anki::anki_server::Anki for AnkiService {
    async fn save_definition(
        &self,
        request: Request<SaveDefinitionRequest>,
    ) -> Result<Response<SaveDefinitionReply>, Status> {
        let res = &request.get_ref().result;
        match res {
            Some(e) => add_to_anki(e).await,
            None => todo!(),
        }

        Ok(Response::new(SaveDefinitionReply {}))
    }
}

async fn add_to_anki(result: &ScanResult) {
    let conf = sample_conf();
    let deck = "test1";
    let model = "Novelcards"; // Model from my collection
    add_note(
        &deck,
        &model,
        &conf
            .into_iter()
            .map(|(n, f)| {
                (
                    n,
                    match f {
                        Field::Expression => ruby_to_expression(&result.ruby),
                        Field::Reading => ruby_to_reading(&result.ruby),
                        Field::Furigana => ruby_to_anki(&result.ruby),
                        Field::Glossary => result.glossary.get(0).unwrap().definition.join("<br>"),
                    },
                )
            })
            .collect(),
    )
    .await;
}

fn sample_conf() -> HashMap<String, Field> {
    let mut conf = HashMap::new();
    conf.insert("Word".to_string(), Field::Expression);
    conf.insert("Reading".to_string(), Field::Reading);
    conf.insert("Furigana".to_string(), Field::Furigana);
    conf.insert("Glossary".to_string(), Field::Glossary);
    conf
}

fn ruby_to_anki(seg: &Vec<RubySegment>) -> String {
    let mut result = String::new();
    for segment in seg {
        result.push_str(&segment.text);
        if let Some(ruby) = &segment.ruby {
            result.push('[');
            result.push_str(&ruby);
            result.push(']');
        }
        result.push(' ');
    }
    result.pop();
    result
}

fn ruby_to_expression(seg: &Vec<RubySegment>) -> String {
    seg.iter()
        .map(|e| e.text.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn ruby_to_reading(seg: &Vec<RubySegment>) -> String {
    seg.iter()
        .map(|e| match &e.ruby {
            Some(e) => e.to_string(),
            None => e.text.clone(),
        })
        .collect::<Vec<_>>()
        .join("")
}
