use std::time::Instant;

use axum::Json;
use reqwest::Client;
use serde::Serialize;

const CP_API_URL: &str = "https://api-gateway.cp.pt/cp/services/travel-api";
const IP_API_URL: &str = "https://www.infraestruturasdeportugal.pt";
const TIMEOUT_MS: u64 = 5000;

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

pub async fn diagnostics() -> Json<DiagnosticsResponse> {
    let cp_status = check_api_health(CP_API_URL).await;
    let ip_status = check_api_health(IP_API_URL).await;

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

async fn check_api_health(url: &str) -> ApiStatus {
    let client = Client::builder()
        .timeout(std::time::Duration::from_millis(TIMEOUT_MS))
        .build()
        .unwrap_or_else(|_| Client::new());

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
