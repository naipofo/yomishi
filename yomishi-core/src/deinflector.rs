use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeinflectionRule {
    pub kana_in: String,
    pub kana_out: String,
    pub rules_in: Vec<Value>,
    pub rules_out: Vec<String>,
}

type DeinflectionList = HashMap<String, Vec<DeinflectionRule>>;

#[derive(Debug, Clone, Default)]
pub struct Deinflection {
    term: String,
    rules: Vec<String>,
    reasons: Vec<String>,
}
fn deinflect_inner(deinf_list: &DeinflectionList, source: Deinflection) -> Vec<Deinflection> {
    deinf_list
        .iter()
        .flat_map(|(reason, variants)| {
            variants.iter().flat_map(
                |DeinflectionRule {
                     kana_in,
                     kana_out,
                     rules_in,
                     rules_out,
                 }| {
                    let Deinflection {
                        term,
                        rules,
                        reasons,
                    } = &source;
                    // TODO: use some kind of unicode libary to make the code more manageable
                    if (rules.len() != 0 && rules != rules_in)
                        || !term.ends_with(kana_in)
                        || (term.char_indices().count() - kana_in.char_indices().count()
                            + kana_out.char_indices().count())
                            <= 0
                    {
                        vec![]
                    } else {
                        deinflect_inner(
                            deinf_list,
                            Deinflection {
                                term: format!(
                                    "{}{}",
                                    term[..term
                                        .char_indices()
                                        .nth(
                                            term.char_indices().count()
                                                - kana_in.char_indices().count()
                                        )
                                        .unwrap()
                                        .0]
                                        .to_string(),
                                    kana_out
                                ),
                                rules: rules_out.clone(),
                                reasons: vec![reason.clone()]
                                    .into_iter()
                                    .chain(reasons.iter().cloned())
                                    .collect(),
                            },
                        )
                    }
                },
            )
        })
        .collect::<Vec<_>>()
        .into_iter()
        .chain(vec![source].into_iter())
        .collect()
}

pub fn deinflect(deinf_list: &DeinflectionList, source: &str) -> Vec<Deinflection> {
    deinflect_inner(
        deinf_list,
        Deinflection {
            term: source.to_string(),
            ..Default::default()
        },
    )
}
