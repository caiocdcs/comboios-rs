use std::sync::Arc;
use std::time::Instant;

use axum::Json;
use axum::extract::State;
use reqwest::Client;
use serde::Serialize;

use crate::domain::AppState;

#[derive(Serialize)]
pub struct DiagnosticsResponse {
    status: String,
    timestamp: String,
    apis: ApiStatuses,
}

#[derive(Serialize)]
pub struct ApiStatuses {
    cp: ApiStatus,
    ip: ApiStatus,
}

#[derive(Serialize)]
pub struct ApiStatus {
    reachable: bool,
    response_time_ms: Option<u64>,
    status_code: Option<u16>,
    error: Option<String>,
}

pub async fn diagnostics(State(state): State<Arc<AppState>>) -> Json<DiagnosticsResponse> {
    let timeout = state.settings.diagnostics_timeout;
    let cp_url = state.settings.cp_api_url.clone();
    let ip_url = state.settings.ip_api_url.clone();

    let client = Client::builder()
        .timeout(timeout)
        .build()
        .unwrap_or_else(|_| Client::new());

    let (cp_status, ip_status) = tokio::join!(
        check_api_health(&client, &cp_url),
        check_api_health(&client, &ip_url)
    );

    let overall_status = if cp_status.reachable || ip_status.reachable {
        "ok"
    } else {
        "degraded"
    };

    Json(DiagnosticsResponse {
        status: overall_status.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        apis: ApiStatuses {
            cp: cp_status,
            ip: ip_status,
        },
    })
}

async fn check_api_health(client: &Client, url: &str) -> ApiStatus {
    let start = Instant::now();

    match client.get(url).send().await {
        Ok(response) => {
            let elapsed = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
            ApiStatus {
                reachable: true,
                response_time_ms: Some(elapsed),
                status_code: Some(response.status().as_u16()),
                error: None,
            }
        }
        Err(e) => ApiStatus {
            reachable: false,
            response_time_ms: None,
            status_code: None,
            error: Some(e.to_string()),
        },
    }
}
