pub mod search;

use std::path::Path;

use crate::{
    database::Database, deinflector::Deinflector, dict::import_from_directory, error::Result,
};

pub struct Dictionary {
    storage: Database,
    deinflector: Deinflector,
}

impl Dictionary {
    pub fn new() -> Result<Self> {
        Ok(Self {
            storage: Database::new(construct_deinflector()?)?,
            deinflector: construct_deinflector()?,
        })
    }

    pub async fn import_dicts(&mut self, p: &Path) {
        let dicts =
            import_from_directory(p, |index| !self.storage.dict_exists(index).unwrap()).unwrap();
        for d in dicts {
            self.storage.load(d).unwrap();
        }
    }
}

// TODO: use deinflection from somewhere else
/// temp fucntion that uses a local file
fn construct_deinflector() -> Result<Deinflector> {
    Ok(Deinflector::new_from_str(include_str!(
        "../../local_test_files/deinflect.json"
    ))?)
}
