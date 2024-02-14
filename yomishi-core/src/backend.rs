use std::path::Path;

use crate::{
    database::Database, deinflector::Deinflector, dict::import_from_directory, error::Result,
};

pub struct Backend {
    pub storage: Database,
    pub deinflector: Deinflector,
}

impl Backend {
    pub async fn new() -> Result<Self> {
        let mut storage = Database::new().await?;
        let dicts = import_from_directory(Path::new("local_test_files/dic"), |index| {
            !storage.dict_exists(index).unwrap()
        })?;
        for d in dicts {
            storage.load(d).unwrap();
        }
        Ok(Self {
            storage,
            deinflector: construct_deinflector()?,
        })
    }
}

fn construct_deinflector() -> Result<Deinflector> {
    Ok(Deinflector::new_from_str(include_str!(
        "../../local_test_files/deinflect.json"
    ))?)
}
