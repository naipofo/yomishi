use std::collections::{BTreeMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    backend::Backend,
    deinflector::{DeinflectionMeta, DeinflectionResult},
    dict::parser::{tag::Tag, term::Term, term_meta::TermMeta},
    error::{Result, YomishiError},
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
    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TermWithTags {
    pub term: Term,
    pub tags: Vec<Tag>,
}

impl Backend {
    pub fn search(&self, text: &str) -> Result<Vec<SearchResult>> {
        self.deinflector
            .deinflect(text)
            .into_iter()
            .map(|DeinflectionResult(term, meta)| {
                struct LookupResult(Term, Vec<Tag>, Vec<Tag>, String);

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

                let lookup_raw = self
                    .storage
                    .get_terms(&term)?
                    .into_iter()
                    .map(|(term, dict_id)| {
                        let tags = self.storage.get_tag_list(&term.definition_tags, &dict_id)?;
                        let term_tags = self.storage.get_tag_list(&term.term_tags, &dict_id)?;

                        Ok(LookupResult(
                            term,
                            tags,
                            term_tags,
                            self.storage.get_dict_by_id(&dict_id)?,
                        ))
                    })
                    .collect::<Result<_>>()?;

                let terms_grouped = group_pairs(lookup_raw);

                terms_grouped
                    .into_iter()
                    .map(|e| {
                        let mut all_tags = HashSet::new();

                        let t = &e.get(0).ok_or(YomishiError::Database)?.0;
                        let term_meta = self.storage.get_term_meta(&t.expression, &t.reading)?;
                        Ok(SearchResult {
                            deinflection: meta.clone(),
                            glossaries: e
                                .into_iter()
                                .map(|LookupResult(term, tags, global, dictionary)| {
                                    all_tags.extend(global);
                                    DictionaryTagged {
                                        data: TermWithTags { term, tags },
                                        dictionary,
                                    }
                                })
                                .collect(),
                            tags: all_tags.into_iter().collect(),
                            meta: term_meta,
                        })
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<_>>>>()
            .map(|e| e.into_iter().flatten().collect())
    }
}
