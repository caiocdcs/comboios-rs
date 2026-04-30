use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    domain::{AppResponse, AppState},
    error::AppError,
};

/// # Errors
///
/// Returns [`AppError`] if the credential refresh fails.
#[tracing::instrument(skip(state))]
pub async fn refresh_credentials(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AppResponse<String>>, AppError> {
    tracing::info!("Manual credential refresh requested");

    state.api.refresh_credentials_from_website().await?;

    Ok(Json(AppResponse {
        data: "Credentials refreshed successfully".to_string(),
    }))
}
