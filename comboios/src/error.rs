use serde_json::Error as SerdeError;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("failed to parse response: {0}")]
    ParseError(#[from] SerdeError),
    #[error("API error (status {status}): {message}")]
    ApiError { status: u16, message: String },
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
