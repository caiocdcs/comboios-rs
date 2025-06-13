use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use comboios::domain::station::Station;
use serde::Deserialize;

use crate::{
    domain::{AppResponse, AppState},
    error::AppError,
};

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    query: String,
}

#[tracing::instrument(skip(state))]
pub async fn stations(
    State(state): State<Arc<AppState>>,
    search: Query<SearchParams>,
) -> Result<Json<AppResponse<Vec<Station>>>, AppError> {
    tracing::info!("Finding stations");

    let response = state.api.get_stations(&search.query).await?;

    Ok(Json(AppResponse {
        data: response.response,
    }))
}
