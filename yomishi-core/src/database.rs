use crate::{deinflector::DeinflectionResult, dict::parser::term::Term};

pub mod slow_inmem;

pub trait Database {
    fn load(&mut self, title: String, terms: Vec<Term>) -> Option<()>;
    fn search<'a>(&'a self, text: &'a str) -> Vec<SearchResult<'a>>;
}

#[derive(Debug)]
pub struct SearchResult<'a>(pub Term, pub DeinflectionResult<'a>);
