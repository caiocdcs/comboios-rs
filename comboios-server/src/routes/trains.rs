use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;

use crate::{
    domain::{AppResponse, AppState, TrainId},
    error::AppError,
};
use chrono::Local;
use comboios_core::domain::journey::TrainJourney;

#[derive(Debug, Deserialize)]
pub struct JourneyQuery {
    pub date: Option<String>,
}

/// # Errors
///
/// Returns [`AppError`] if the request handling fails.
#[tracing::instrument]
pub async fn trains(
    State(state): State<Arc<AppState>>,
    Path(train_id): Path<TrainId>,
) -> Result<Json<AppResponse<String>>, AppError> {
    let _ = (state, train_id);
    tracing::info!("Train details endpoint - deprecated");

    Ok(Json(AppResponse {
        data: "Train details API is no longer available. Please use station timetable instead."
            .to_string(),
    }))
}

/// # Errors
///
/// Returns [`AppError`] if the CP or IP API call fails.
#[tracing::instrument]
pub async fn get_train_journey(
    State(state): State<Arc<AppState>>,
    Path(train_id): Path<String>,
    Query(query): Query<JourneyQuery>,
) -> Result<Json<AppResponse<TrainJourney>>, AppError> {
    tracing::info!("Fetching train journey for {train_id}");

    let date = query
        .date
        .unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
    let train = state.api.get_train_journey(&train_id, &date).await?;

    Ok(Json(AppResponse { data: train }))
}
