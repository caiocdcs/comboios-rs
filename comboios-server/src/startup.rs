use std::{sync::Arc, time::Duration};

use anyhow::Result;
use axum::{BoxError, Router, error_handling::HandleErrorLayer, routing::get};
use reqwest::{Client, StatusCode};
use tokio::net::TcpListener;
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
        health_check::health_check, station_timetables::station_timetables, stations::stations,
        trains::trains,
    },
};

pub async fn run(listener: TcpListener) -> Result<()> {
    let client = Client::new();

    let app_state = Arc::new(AppState { client });

    let app = Router::new()
        .route("/ping", get(health_check))
        .route("/stations/{station_name}", get(stations))
        .route("/stations/timetable/{station_id}", get(station_timetables))
        .route("/trains/{train_id}", get(trains))
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid::default())
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

async fn handle_errors(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        tracing::error!("{:?}", err);
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {err}"),
        )
    }
}
