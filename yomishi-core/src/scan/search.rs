use std::{
    collections::{BTreeMap, HashSet},
    vec,
};

use futures::future::join_all;
use serde::{Deserialize, Serialize};
use yomishi_config::SerdeKeys::DictionariesDisabled;

use crate::{
    backend::Backend,
    database::terms::LookupData,
    deinflector::DeinflectionMeta,
    dict::parser::{tag::Tag, term::Term, term_meta::TermMeta},
    error::{yo_er, Result, YomishiError},
};

#[derive(Deserialize, Serialize)]
pub struct SearchResult {
    pub deinflection: DeinflectionMeta,
    pub glossaries: Vec<DictionaryTagged<TermWithTags>>,
    pub tags: Vec<Tag>,
    pub meta: Vec<DictionaryTagged<TermMeta>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DictionaryTagged<T> {
    pub dictionary: String,
    pub dictionary_id: String,
    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TermWithTags {
    pub term: Term,
    pub tags: Vec<Tag>,
}

fn group_pairs_new(v: Vec<LookupData>) -> Vec<Vec<LookupData>> {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct DedupKey(String, String);
    // TODO: Should probably be ref instead of value

    v.into_iter()
        .fold(BTreeMap::<_, Vec<LookupData>>::new(), |mut acc, el| {
            acc.entry(DedupKey(el.expression.to_string(), el.reading.to_string()))
                .or_default()
                .push(el);
            acc
        })
        .into_values()
        .collect()
}

#[derive(Deserialize, Serialize)]
pub struct SearchResultNew {
    pub deinflection: DeinflectionMeta,
    pub glossaries: Vec<DictionaryTagged<TermWithTags>>,
    pub tags: Vec<Tag>,
    pub meta: Vec<DictionaryTagged<TermMeta>>,
}

impl Backend {
    pub async fn search(&self, text: &str) -> Result<Vec<SearchResult>> {
        let disabled_dicts: Vec<String> =
            serde_json::from_value(self.storage.get_serde(DictionariesDisabled).await)?;

        let deinflections = self.deinflector.deinflect(text);

        let terms = self
            .storage
            .new_term_lookup(
                deinflections.iter().map(|t| t.0.as_str()).collect(),
                &disabled_dicts,
            )
            .await?;

        // TODO: consider grouping directly in surql (?)
        let terms_grouped = group_pairs_new(terms);

        let get_deinf_meta = |LookupData { expression, .. }: &LookupData| {
            deinflections
                .iter()
                .find(|d| d.0 == *expression)
                .unwrap()
                .1
                .clone()
        };

        join_all(terms_grouped.into_iter().map(|e| async move {
            let mut all_tags = HashSet::new();

            let shared_term = &e.first().ok_or(yo_er!())?;
            let meta = self.storage.get_term_meta("term", "reading").await?;

            Ok::<SearchResult, YomishiError>(SearchResult {
                deinflection: get_deinf_meta(shared_term),
                glossaries: e
                    .into_iter()
                    .map(
                        |LookupData {
                             dictionary,
                             dictionary_name,
                             expression,
                             reading,
                             glossary,
                             rules,
                             tags,
                             definition_tags,
                         }| {
                            all_tags.extend(definition_tags);
                            // TODO: instead of putting mock data, rework how
                            // data is moved around when scaning
                            DictionaryTagged {
                                data: TermWithTags {
                                    term: Term {
                                        expression,
                                        reading,
                                        definition_tags: vec![],
                                        rules,
                                        score: 0,
                                        glossary: serde_json::from_str(&glossary).unwrap(),
                                        sequence: 0,
                                        term_tags: vec![],
                                    },
                                    tags: tags.into_iter().map(|t| t.into_model()).collect(),
                                },
                                dictionary: dictionary_name,
                                dictionary_id: dictionary.id.to_string(),
                            }
                        },
                    )
                    .collect(),
                tags: all_tags.into_iter().map(|e| e.into_model()).collect(),
                meta,
            })
        }))
        .await
        .into_iter()
        .collect()
    }
}
