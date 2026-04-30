//! Tests for error handling

use comboios_core::error::CoreError;

#[test]
fn test_api_error_display() {
    let error = CoreError::ApiError {
        status: 404,
        message: "Train not found".to_string(),
    };
    let display = format!("{}", error);
    assert!(display.contains("404"));
    assert!(display.contains("Train not found"));
}

#[test]
fn test_api_error_500() {
    let error = CoreError::ApiError {
        status: 500,
        message: "Internal server error".to_string(),
    };
    let display = format!("{}", error);
    assert!(display.contains("500"));
}

#[test]
fn test_invalid_input_display() {
    let error = CoreError::InvalidInput("Station ID cannot be empty".to_string());
    let display = format!("{}", error);
    assert!(display.contains("invalid input"));
    assert!(display.contains("Station ID cannot be empty"));
}

#[test]
fn test_invalid_input_empty() {
    let error = CoreError::InvalidInput(String::new());
    let display = format!("{}", error);
    assert!(display.contains("invalid input"));
}

#[test]
fn test_error_debug() {
    let error = CoreError::ApiError {
        status: 401,
        message: "Unauthorized".to_string(),
    };
    let debug = format!("{:?}", error);
    assert!(debug.contains("ApiError"));
    assert!(debug.contains("401"));
}
