use std::collections::{BTreeMap, HashSet};

use futures::future::join_all;
use serde::{Deserialize, Serialize};
use yomishi_config::SerdeKeys::DictionariesDisabled;

use crate::{
    backend::Backend,
    deinflector::{DeinflectionMeta, DeinflectionResult},
    dict::parser::{tag::Tag, term::Term, term_meta::TermMeta},
    error::{yo_er, Result},
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
impl Backend {
    pub async fn search(&self, text: &str) -> Result<Vec<SearchResult>> {
        let disabled_dicts: Vec<String> =
            serde_json::from_value(self.storage.get_serde(DictionariesDisabled).await)?;

        join_all(
            self.deinflector
                .deinflect(text)
                .iter()
                .map(|d| self.search_single(d, &disabled_dicts)),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<Vec<_>>>>()
        .map(|e| e.into_iter().flatten().collect())
    }

    async fn search_single(
        &self,
        DeinflectionResult(term, meta): &DeinflectionResult,
        disabled_dicts: &[String],
    ) -> Result<Vec<SearchResult>> {
        let lookup_raw = join_all(
            self.storage
                .get_terms(term)
                .await?
                .into_iter()
                .filter(|(_, x)| !disabled_dicts.contains(x))
                .map(|r| async {
                    let (term, dict_id) = r;
                    let tags = self
                        .storage
                        .get_tag_list(&term.definition_tags, &dict_id)
                        .await?;
                    let term_tags = self.storage.get_tag_list(&term.term_tags, &dict_id).await?;

                    Ok(LookupResult(
                        term,
                        tags,
                        term_tags,
                        self.storage.get_dict_by_id(&dict_id).await?,
                        dict_id,
                    ))
                }),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

        let terms_grouped = LookupResult::group_pairs(lookup_raw);

        join_all(terms_grouped.into_iter().map(|e| async move {
            let mut all_tags = HashSet::new();

            let t = &e.first().ok_or(yo_er!())?.0;
            let term_meta: Vec<_> = self
                .storage
                .get_term_meta(&t.expression, &t.reading)
                .await?
                .into_iter()
                .filter(|DictionaryTagged { dictionary_id, .. }| {
                    !disabled_dicts.contains(dictionary_id)
                })
                .collect();
            Ok(SearchResult {
                deinflection: meta.clone(),
                glossaries: e
                    .into_iter()
                    .map(
                        |LookupResult(term, tags, global, dictionary, dictionary_id)| {
                            all_tags.extend(global);
                            DictionaryTagged {
                                data: TermWithTags { term, tags },
                                dictionary,
                                dictionary_id,
                            }
                        },
                    )
                    .collect(),
                tags: all_tags.into_iter().collect(),
                meta: term_meta,
            })
        }))
        .await
        .into_iter()
        .collect()
    }
}

struct LookupResult(Term, Vec<Tag>, Vec<Tag>, String, String);

impl LookupResult {
    fn group_pairs(v: Vec<LookupResult>) -> Vec<Vec<LookupResult>> {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        struct DedupKey(String, String);

        v.into_iter()
            .fold(BTreeMap::<_, Vec<LookupResult>>::new(), |mut acc, el| {
                acc.entry(DedupKey(
                    el.0.expression.to_string(),
                    el.0.reading.to_string(),
                ))
                .or_default()
                .push(el);
                acc
            })
            .into_values()
            .collect()
    }
}
