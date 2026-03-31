use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use comboios::domain::station_timetable::StationBoard;
use chrono::Local;

use crate::{
    domain::{AppResponse, AppState},
    error::AppError,
};

#[tracing::instrument(skip(state))]
pub async fn station_timetables(
    State(state): State<Arc<AppState>>,
    Path(station_id): Path<String>,
) -> Result<Json<AppResponse<Vec<StationBoard>>>, AppError> {
    tracing::info!("Finding timetable for station {}", station_id);

    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();
    let start_time = now.format("%H:%M").to_string();

    let boards = state.api.get_station_timetable(&station_id, &date, Some(&start_time)).await?;

    Ok(Json(AppResponse { data: boards.response }))
}
