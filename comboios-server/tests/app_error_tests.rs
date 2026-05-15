//! Tests for AppError response format.
//!
//! Verifies that each error variant produces the expected HTTP status code
//! and JSON error body shape.

use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use comboios_core::error::CoreError;
use comboios_server::error::AppError;

/// Parse the JSON error body returned by AppError.
async fn parse_error_body(response: axum::http::Response<Body>) -> serde_json::Value {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn test_api_error_4xx_returns_client_status() {
    let err = AppError::CoreError(CoreError::ApiError {
        status: 404,
        message: "Train not found".to_string(),
    });

    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = parse_error_body(response).await;
    assert_eq!(body["error_type"], "ApiError");
    assert_eq!(body["status"], 404);
    assert!(body["error"].as_str().unwrap().contains("Train not found"));
}

#[tokio::test]
async fn test_api_error_5xx_returns_server_status() {
    let err = AppError::CoreError(CoreError::ApiError {
        status: 503,
        message: "Service unavailable".to_string(),
    });

    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let body = parse_error_body(response).await;
    assert_eq!(body["error_type"], "ApiError");
    assert_eq!(body["status"], 503);
}

#[tokio::test]
async fn test_invalid_input_returns_bad_request() {
    let err = AppError::CoreError(CoreError::InvalidInput("bad input".to_string()));

    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = parse_error_body(response).await;
    assert_eq!(body["error_type"], "InvalidInput");
    assert!(body["error"].as_str().unwrap().contains("bad input"));
}

#[tokio::test]
async fn test_invalid_train_id_returns_bad_request() {
    let err = AppError::InvalidTrainId;

    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = parse_error_body(response).await;
    assert_eq!(body["error_type"], "InvalidTrainId");
    assert_eq!(body["status"], 400);
}

#[tokio::test]
async fn test_parse_error_returns_bad_gateway() {
    let err = AppError::CoreError(CoreError::ParseError(
        serde_json::from_str::<i64>("not a number").unwrap_err(),
    ));

    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);

    let body = parse_error_body(response).await;
    assert_eq!(body["error_type"], "ParseError");
}
