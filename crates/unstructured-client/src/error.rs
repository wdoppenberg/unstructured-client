use std::result::Result as BaseResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Network error: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse URL: {0}")]
    URLParseFailed(String),

    #[error("Text extraction failed: {0}")]
    ExtractionFailed(String),

    #[error("Metadata field not present: {0}")]
    MetadataFieldNotPresent(String),

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("File IO error: {0}")]
    FileIOError(String),

    #[error("Timeout occurred")]
    Timeout,

    #[error("Unexpected response from service: {0}")]
    UnexpectedResponse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = BaseResult<T, ClientError>;
