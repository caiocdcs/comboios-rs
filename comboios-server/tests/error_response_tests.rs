//! Integration tests for server route behaviour that does not
//! require a live Comboios client.

use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode},
    routing::get,
};
use tower::ServiceExt;

use comboios_server::routes::health_check::health_check;

fn health_app() -> Router {
    Router::new().route("/ping", get(health_check))
}

// ---------------------------------------------------------------------------
// Health check
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_health_check_returns_pong() {
    let app = health_app();

    let response = app
        .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(&body[..], b"PONG");
}

#[tokio::test]
async fn test_health_check_rejects_post() {
    let app = health_app();

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/ping")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

// ---------------------------------------------------------------------------
// 404 for unknown routes
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_not_found_route() {
    let app = health_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_not_found_for_known_path_wrong_method() {
    let app = health_app();

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/ping")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

// ---------------------------------------------------------------------------
// Root path returns 404
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_root_returns_not_found() {
    let app = health_app();

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
