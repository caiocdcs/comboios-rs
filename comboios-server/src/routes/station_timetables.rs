use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use comboios::domain::station_timetable::StationBoard;

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

    let boards = state.api.get_station_board_now(&station_id).await?;

    Ok(Json(AppResponse { data: boards }))
}
