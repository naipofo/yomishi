use crate::dict::parser::kanji_meta::KanjiMeta;

use super::Database;

impl Database {
    pub fn insert_kanjis_meta_bulk(&self, _terms: Vec<KanjiMeta>, _dictionary_idd: &str) {
        // NO OP
    }
}
