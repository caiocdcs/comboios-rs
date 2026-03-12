use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    domain::{AppResponse, AppState, TrainId},
    error::AppError,
};

#[tracing::instrument]
pub async fn trains(
    State(_state): State<Arc<AppState>>,
    Path(_train_id): Path<TrainId>,
) -> Result<Json<AppResponse<String>>, AppError> {
    tracing::info!("Train details endpoint - deprecated");

    // This endpoint is deprecated as the CP API is no longer available
    Ok(Json(AppResponse {
        data: "Train details API is no longer available. Please use station timetable instead.".to_string(),
    }))
}
