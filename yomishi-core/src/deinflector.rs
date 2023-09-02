mod step_search;

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use self::step_search::StepSearch;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeinflectionRule {
    pub kana_in: String,
    pub kana_out: String,
    pub rules_in: Vec<Value>,
    pub rules_out: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct DeinflectionResult<'a>(pub String, pub DeinflectionMeta<'a>);

#[derive(Debug, Default, Clone)]
pub struct DeinflectionMeta<'a> {
    pub source: &'a str,
    pub rules: Vec<&'a str>,
    pub reasons: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Deinflector(HashMap<String, Vec<DeinflectionRule>>);

impl Deinflector {
    pub fn new_from_str(json: &str) -> serde_json::Result<Self> {
        Ok(Self(serde_json::from_str(json)?))
    }
    pub fn deinflect<'a>(&'a self, source: &'a str) -> Vec<DeinflectionResult> {
        let mut deinflections: Vec<DeinflectionResult<'_>> = vec![];

        deinflections.extend(StepSearch::new_from_str(source).flat_map(|source| {
            self.deinflect_single(DeinflectionResult(
                source.to_string(),
                DeinflectionMeta {
                    source,
                    ..Default::default()
                },
            ))
        }));

        deinflections.sort_by(|a, b| a.0.len().cmp(&b.0.len()));
        deinflections.dedup_by(|a, b| a.0 == b.0);

        deinflections
    }

    fn deinflect_single<'a>(&'a self, text: DeinflectionResult<'a>) -> Vec<DeinflectionResult<'a>> {
        self.0
            .iter()
            .flat_map(|(reason, rules)| {
                rules.iter().flat_map(
                    |DeinflectionRule {
                         kana_in,
                         kana_out,
                         rules_in,
                         rules_out,
                     }| {
                        let DeinflectionResult(
                            term,
                            DeinflectionMeta {
                                source: _,
                                rules,
                                reasons,
                            },
                        ) = &text;

                        if rules.len() != 0 && &rules != &rules_in
                            || !term.ends_with(kana_in)
                            || (term.char_indices().count() - kana_in.char_indices().count()
                                + kana_out.char_indices().count())
                                <= 0
                        {
                            vec![]
                        } else {
                            let deinf_term = format!(
                                "{}{}",
                                &term[..term
                                    // TODO: should not have to use char_indices here
                                    .char_indices()
                                    .nth(
                                        term.char_indices().count()
                                            - kana_in.char_indices().count()
                                    )
                                    .unwrap()
                                    .0],
                                kana_out
                            );
                            self.deinflect_single(DeinflectionResult(
                                deinf_term,
                                DeinflectionMeta {
                                    source: text.1.source,
                                    rules: rules_out.iter().map(|s| s.as_str()).collect(),
                                    reasons: vec![reason.as_str()]
                                        .into_iter()
                                        .chain(reasons.clone())
                                        .collect(),
                                },
                            ))
                        }
                    },
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
            .chain(vec![text])
            .collect()
    }
}
