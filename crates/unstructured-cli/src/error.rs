use thiserror::Error;
use unstructured_client::error::ClientError;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Unstructured client error: {0}")]
    ClientError(#[from] ClientError),

    #[error("JSON error: {0}")]
    JSONError(#[from] serde_json::Error),
}
