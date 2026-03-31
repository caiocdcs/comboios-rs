use axum::{
    response::{IntoResponse, Response},
    Json,
};
use comboios::error::CoreError;
use reqwest::StatusCode;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("error getting data")]
    CoreError(#[from] CoreError),
    #[error("invalid train id")]
    InvalidTrainId,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    error_type: String,
    status: u16,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            AppError::CoreError(core_err) => match core_err {
                CoreError::ApiError { status, message } => {
                    if *status >= 500 {
                        (
                            StatusCode::from_u16(*status)
                                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                            "ApiError".to_string(),
                            message.clone(),
                        )
                    } else {
                        (
                            StatusCode::from_u16(*status).unwrap_or(StatusCode::BAD_REQUEST),
                            "ApiError".to_string(),
                            message.clone(),
                        )
                    }
                }
                CoreError::NetworkError(_) => (
                    StatusCode::BAD_GATEWAY,
                    "NetworkError".to_string(),
                    core_err.to_string(),
                ),
                CoreError::ParseError(_) => (
                    StatusCode::BAD_GATEWAY,
                    "ParseError".to_string(),
                    "Failed to parse API response".to_string(),
                ),
                CoreError::InvalidInput(msg) => (
                    StatusCode::BAD_REQUEST,
                    "InvalidInput".to_string(),
                    msg.clone(),
                ),
            },
            AppError::InvalidTrainId => (
                StatusCode::BAD_REQUEST,
                "InvalidTrainId".to_string(),
                "Invalid train ID format".to_string(),
            ),
        };

        tracing::error!("Error: {} ({})", message, error_type);

        let body = ErrorResponse {
            error: message,
            error_type,
            status: status.as_u16(),
        };

        (status, Json(body)).into_response()
    }
}
