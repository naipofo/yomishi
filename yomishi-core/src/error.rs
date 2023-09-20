pub type Result<T, E = YomishiError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum YomishiError {
    Database,
    Json,
}

impl From<serde_json::Error> for YomishiError {
    fn from(_: serde_json::Error) -> Self {
        YomishiError::Json
    }
}

impl From<rusqlite::Error> for YomishiError {
    fn from(_: rusqlite::Error) -> Self {
        YomishiError::Database
    }
}
