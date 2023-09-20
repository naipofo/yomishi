use crate::{database::SearchResult, error::Result};

use super::Dictionary;

impl Dictionary {
    pub fn search(&mut self, text: &str) -> Result<Vec<SearchResult>> {
        Ok(self.storage.search(text)?)
    }
}
