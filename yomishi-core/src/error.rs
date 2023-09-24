use tonic::{Code, Status};

pub type Result<T, E = YomishiError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum YomishiError {
    Database,
    Json,
    Request,
    Decode,
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

impl Into<Status> for YomishiError {
    fn into(self) -> Status {
        Status::new(Code::Internal, format!("{:?}", self))
    }
}
