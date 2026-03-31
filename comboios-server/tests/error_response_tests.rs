//! Tests for server error responses

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use tower::ServiceExt;

use comboios_server::routes::health_check::health_check;

fn app() -> Router {
    Router::new().route("/ping", axum::routing::get(health_check))
}

#[tokio::test]
async fn test_health_check_returns_pong() {
    let app = app();

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
async fn test_not_found_route() {
    let app = app();

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
