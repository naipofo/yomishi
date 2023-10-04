pub type Result<T, E = YomishiError> = std::result::Result<T, E>;

// TODO: use something for error messages
#[derive(Debug)]
pub enum YomishiError {
    Database,
    Json,
    Request,
    Decode,
    IOError,
    Zip,
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

impl From<reqwest::Error> for YomishiError {
    fn from(_: reqwest::Error) -> Self {
        YomishiError::Request
    }
}

impl From<prost::DecodeError> for YomishiError {
    fn from(_: prost::DecodeError) -> Self {
        YomishiError::Decode
    }
}

impl From<std::io::Error> for YomishiError {
    fn from(_: std::io::Error) -> Self {
        YomishiError::IOError
    }
}

impl From<zip::result::ZipError> for YomishiError {
    fn from(_: zip::result::ZipError) -> Self {
        YomishiError::Zip
    }
}
