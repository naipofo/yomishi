use crate::{
    deinflector::{deinflect_all, DeinflectionList},
    dict_parser::Term,
};

use super::Database;

/// Meant for testing, not real word use. A stand-in for a real database
#[derive(Debug, Default)]
pub struct SlowInMemeoryDatabase {
    dicts: Vec<(String, Vec<Term>)>,
    deinf: DeinflectionList,
}

impl SlowInMemeoryDatabase {
    pub fn new(deinf: DeinflectionList) -> Self {
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

    fn search(&self, text: &str) -> Vec<Term> {
        deinflect_all(&self.deinf, text)
            .iter()
            .map(|term| self.search_all(&term.1.term))
            .flatten()
            .cloned()
            .collect()
    }
}
