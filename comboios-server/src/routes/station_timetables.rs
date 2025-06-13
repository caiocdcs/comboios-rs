use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use comboios::domain::station_timetable::Timetable;

use crate::{
    domain::{AppResponse, AppState},
    error::AppError,
};

#[tracing::instrument(skip(state))]
pub async fn station_timetables(
    State(state): State<Arc<AppState>>,
    Path(station_id): Path<String>,
) -> Result<Json<AppResponse<Vec<Timetable>>>, AppError> {
    tracing::info!("Finding timetable for station");

    let timetables = state.api.get_station_timetable(&station_id).await?;

    Ok(Json(AppResponse { data: timetables }))
}
