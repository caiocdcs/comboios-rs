use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use comboios::domain::station::Station;

use crate::{
    domain::{AppResponse, AppState},
    error::AppError,
};

#[tracing::instrument(skip(state))]
pub async fn stations(
    State(state): State<Arc<AppState>>,
    Path(station_name): Path<String>,
) -> Result<Json<AppResponse<Vec<Station>>>, AppError> {
    tracing::info!("Finding stations");

    let response = comboios::client::get_stations(state.client.clone(), &station_name).await?;

    Ok(Json(AppResponse {
        data: response.response,
    }))
}
