use crate::dict::parser::kanji::Kanji;

use super::Database;

impl Database {
    pub fn insert_kanjis_bulk(&self, _terms: Vec<Kanji>, _dictionary_idd: &str) {
        // TODO: Figure out somethig smarter than a "kanji import"
        // kanji data should be built in or sourced from a single source

        // NO OP
    }
}
