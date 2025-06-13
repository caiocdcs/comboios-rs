use axum::response::{IntoResponse, Response};
use comboios::error::CoreError;
use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("error getting data")]
    CoreError(#[from] CoreError),
    #[error("invalid train id")]
    InvalidTrainId,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::CoreError(core_error) => {
                tracing::error!("{:?}", core_error);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AppError::InvalidTrainId => {
                tracing::error!("{:?}", self);
                StatusCode::BAD_REQUEST.into_response()
            }
        }
    }
}
