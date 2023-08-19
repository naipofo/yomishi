use crate::dict_parser::Term;

pub mod slow_inmem;

pub trait Database {
    fn load(&mut self, title: String, terms: Vec<Term>) -> Option<()>;
    fn search(&self, text: &str) -> Vec<Term>;
}
