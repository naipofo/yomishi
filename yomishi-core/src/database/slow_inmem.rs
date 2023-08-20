use crate::{deinflector::Deinflector, dict_parser::Term};

use super::{Database, SearchResult};

/// Meant for testing, not real word use. A stand-in for a real database
#[derive(Debug)]
pub struct SlowInMemeoryDatabase {
    dicts: Vec<(String, Vec<Term>)>,
    deinf: Deinflector,
}

impl SlowInMemeoryDatabase {
    pub fn new(deinf: Deinflector) -> Self {
        SlowInMemeoryDatabase {
            dicts: vec![],
            deinf,
        }
    }

    fn search_all(&self, term: &str) -> Vec<&Term> {
        self.dicts
            .iter()
            .filter_map(|(_, terms)| {
                Some(
                    terms
                        .iter()
                        .filter(|e| e.expression == term)
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
            .collect()
    }
}
impl Database for SlowInMemeoryDatabase {
    fn load(&mut self, name: String, terms: Vec<Term>) -> Option<()> {
        self.dicts.push((name, terms));
        Some(())
    }

    fn search<'a>(&'a self, text: &'a str) -> Vec<SearchResult<'a>> {
        let binding = self.deinf.deinflect(text);
        let mut r = binding
            .iter()
            .map(|term| {
                self.search_all(&term.term)
                    .into_iter()
                    .map(move |r| (r, term))
            })
            .flatten()
            .collect::<Vec<_>>();
        r.dedup_by(|a, b| a.0.sequence == b.0.sequence);
        r.reverse();

        r.into_iter()
            .map(|(a, b)| SearchResult(a.clone(), b.clone()))
            .collect()
    }
}
