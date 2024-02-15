pub type Result<T, E = YomishiError> = std::result::Result<T, E>;

// TODO: use something for error messages
#[derive(Debug)]
pub struct YomishiError {
    pub detail: String,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    Database,
    Json,
    Request,
    Decode,
    IOError,
    Zip,
    Other,
}

macro_rules! yo_er {
    () => {
        crate::error::YomishiError {
            detail: format!("[{}:{}:{}]", file!(), line!(), column!()),
            kind: crate::error::ErrorKind::Other,
        }
    };
}

pub(crate) use yo_er;

impl From<serde_json::Error> for YomishiError {
    fn from(e: serde_json::Error) -> Self {
        YomishiError {
            detail: format!("{:?}", e),
            kind: ErrorKind::Json,
        }
    }
}

impl From<surrealdb::Error> for YomishiError {
    fn from(e: surrealdb::Error) -> Self {
        YomishiError {
            detail: format!("{:?}", e),
            kind: ErrorKind::Database,
        }
    }
}

impl From<reqwest::Error> for YomishiError {
    fn from(e: reqwest::Error) -> Self {
        YomishiError {
            detail: format!("{:?}", e),
            kind: ErrorKind::Request,
        }
    }
}

impl From<prost::DecodeError> for YomishiError {
    fn from(e: prost::DecodeError) -> Self {
        YomishiError {
            detail: format!("{:?}", e),
            kind: ErrorKind::Decode,
        }
    }
}

impl From<std::io::Error> for YomishiError {
    fn from(e: std::io::Error) -> Self {
        YomishiError {
            detail: format!("{:?}", e),
            kind: ErrorKind::IOError,
        }
    }
}

impl From<zip::result::ZipError> for YomishiError {
    fn from(e: zip::result::ZipError) -> Self {
        YomishiError {
            detail: format!("{:?}", e),
            kind: ErrorKind::Zip,
        }
    }
}
