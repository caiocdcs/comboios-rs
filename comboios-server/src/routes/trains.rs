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
use comboios::domain::journey::TrainJourney;
use chrono::Local;

#[derive(Debug, Deserialize)]
pub struct JourneyQuery {
    pub date: Option<String>,
}

#[tracing::instrument]
pub async fn trains(
    State(_state): State<Arc<AppState>>,
    Path(_train_id): Path<TrainId>,
) -> Result<Json<AppResponse<String>>, AppError> {
    tracing::info!("Train details endpoint - deprecated");

    Ok(Json(AppResponse {
        data: "Train details API is no longer available. Please use station timetable instead.".to_string(),
    }))
}

#[tracing::instrument]
pub async fn get_train_journey(
    State(state): State<Arc<AppState>>,
    Path(train_id): Path<String>,
    Query(query): Query<JourneyQuery>,
) -> Result<Json<AppResponse<TrainJourney>>, AppError> {
    tracing::info!("Fetching train journey for {}", train_id);

    let date = query.date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
    let train = state.api.get_train_journey(&train_id, &date).await?;
    
    Ok(Json(AppResponse { data: train }))
}
