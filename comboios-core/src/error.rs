use serde_json::Error as SerdeError;

/// Top-level error type for all `comboios-core` operations.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    /// A network-level failure occurred while contacting an API endpoint.
    ///
    /// Wraps the underlying [`reqwest::Error`], which may include connection
    /// timeouts, DNS failures, or TLS errors.
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// The API response body could not be deserialized into the expected type.
    ///
    /// Usually indicates an upstream API change or an unexpected response shape.
    #[error("failed to parse response: {0}")]
    ParseError(#[from] SerdeError),

    /// The API returned a non-success HTTP status code.
    ///
    /// `status` is the raw HTTP status (e.g. `401`, `404`, `500`) and `message`
    /// contains the error body or a human-readable description from the server.
    #[error("API error (status {status}): {message}")]
    ApiError { status: u16, message: String },

    /// A caller-supplied argument was rejected before sending any request.
    ///
    /// The inner string describes which parameter is invalid and why.
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
