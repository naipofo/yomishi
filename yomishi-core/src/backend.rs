use std::path::Path;

use tokio::runtime::{Runtime, self};

use crate::{
    database::Database, deinflector::Deinflector, dict::import_from_directory, error::Result,
};

pub struct Backend {
    pub storage: Database,
    pub deinflector: Deinflector,
    pub runtime: Runtime,
}

impl Backend {
    pub fn new() -> Result<Self> {
        let mut storage = Database::new()?;
        let dicts = import_from_directory(Path::new("local_test_files/dic"), |index| {
            !storage.dict_exists(index).unwrap()
        })?;
        for d in dicts {
            storage.load(d).unwrap();
        }

        Ok(Self {
            storage,
            deinflector: construct_deinflector()?,
            runtime: runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap()
        })
    }
}

fn construct_deinflector() -> Result<Deinflector> {
    Ok(Deinflector::new_from_str(include_str!(
        "../../local_test_files/deinflect.json"
    ))?)
}
