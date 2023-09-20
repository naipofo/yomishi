use std::{ops::DerefMut, path::Path};

use tokio::sync::Mutex;

use crate::{dictionary::Dictionary, protos::yomishi::config::Config};

use self::config::default_config;

pub mod anki;
pub mod config;
pub mod scan;

struct ConfigState(pub Config);

pub struct Backend {
    db: Mutex<Dictionary>,
    config: Mutex<ConfigState>,
}

impl Backend {
    pub async fn new() -> Self {
        let mut dict = Dictionary::new().unwrap();

        dict.import_dicts(Path::new("../local_test_files/dic"))
            .await;

        Self {
            db: Mutex::new(dict),
            config: Mutex::new(ConfigState(default_config())),
        }
    }

    pub async fn with_dict<F, T>(&self, func: F) -> T
    where
        F: FnOnce(&mut Dictionary) -> T,
    {
        func(&mut self.db.lock().await.deref_mut())
    }
}
