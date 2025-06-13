// Custom error type to handle different error scenarios
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("error calling cp client")]
    ClientError(#[from] reqwest::Error),
    #[error("error parsing response from client")]
    ParseError,
}
