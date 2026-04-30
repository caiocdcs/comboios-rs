use std::{sync::Arc, time::Duration};

use anyhow::Result;
use axum::{BoxError, Json, Router, error_handling::HandleErrorLayer, routing::get};
use comboios_core::Comboios;
use reqwest::StatusCode;
use serde::Serialize;
use tokio::net::TcpListener;
use tokio::time::interval;
use tower::ServiceBuilder;
use tower_http::{
    ServiceBuilderExt,
    cors::{Any, CorsLayer},
    request_id::MakeRequestUuid,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};

use crate::{
    domain::AppState,
    routes::{
        diagnostics::diagnostics,
        health_check::health_check,
        refresh::refresh_credentials,
        station_timetables::station_timetables,
        stations::stations,
        trains::{get_train_journey, trains},
    },
};

#[derive(Serialize)]
struct ErrorBody {
    error: String,
    error_type: String,
    status: u16,
}

/// # Errors
///
/// Returns an error if CP credentials cannot be fetched on startup, or if the
/// TCP listener fails.
pub async fn run(listener: TcpListener) -> Result<()> {
    let api = Comboios::new().await?;

    tracing::info!("CP credentials loaded from cp.pt on startup");

    let app_state = Arc::new(AppState { api: api.clone() });

    // Background task: refresh CP credentials every 55 minutes
    let api_for_bg = api.clone();
    tokio::spawn(async move {
        let mut refresh_interval = interval(Duration::from_secs(55 * 60));

        // Wait for first interval, then refresh
        refresh_interval.tick().await;

        loop {
            match api_for_bg.refresh_credentials_from_website().await {
                Ok(()) => tracing::info!("Background credential refresh succeeded"),
                Err(e) => tracing::warn!("Background credential refresh failed: {e}"),
            }
            refresh_interval.tick().await;
        }
    });

    let app = Router::new()
        .route("/ping", get(health_check))
        .route("/refresh", get(refresh_credentials))
        .route("/diagnostics", get(diagnostics))
        .route("/stations", get(stations))
        .route("/stations/timetable/{station_id}", get(station_timetables))
        .route("/trains/{train_id}", get(trains))
        .route("/trains/{train_id}/journey", get(get_train_journey))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
                .expose_headers(Any)
                .max_age(Duration::from_secs(86400)),
        )
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .layer(HandleErrorLayer::new(handle_errors))
                .timeout(Duration::from_secs(30))
                .propagate_x_request_id(),
        )
        .with_state(app_state);

    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_errors(err: BoxError) -> (StatusCode, Json<ErrorBody>) {
    let (status, error_type, message) = if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "TimeoutError",
            "Request timed out after 30 seconds".to_string(),
        )
    } else {
        tracing::error!("Unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "InternalError",
            "An unexpected error occurred".to_string(),
        )
    };

    (
        status,
        Json(ErrorBody {
            error: message,
            error_type: error_type.to_string(),
            status: status.as_u16(),
        }),
    )
}
