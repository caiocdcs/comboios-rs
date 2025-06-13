use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use comboios::domain::train::Train;

use crate::{
    domain::{AppResponse, AppState, TrainId},
    error::AppError,
};

#[tracing::instrument(skip(state))]
pub async fn trains(
    State(state): State<Arc<AppState>>,
    Path(train_id): Path<TrainId>,
) -> Result<Json<AppResponse<Train>>, AppError> {
    tracing::info!("Finding train details");

    let train = state.api.get_train_details(train_id.into()).await?;

    Ok(Json(AppResponse { data: train }))
}
