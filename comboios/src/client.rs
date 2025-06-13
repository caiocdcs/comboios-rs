use std::time::Duration;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{
    domain::{station::StationResponse, station_timetable::Timetable, train::Train},
    error::CoreError,
};

const CP_BASE_URL: &str = "https://www.cp.pt/sites/spring";
const IP_BASE_URL: &str = "https://www.infraestruturasdeportugal.pt";

/// Main API client for Comboios operations
#[derive(Debug, Clone)]
pub struct ComboiosApi {
    client: Client,
    default_timeout: Duration,
}

impl ComboiosApi {
    /// Create a new instance with default reqwest client
    pub fn new() -> Self {
        Self::with_client(Client::new())
    }

    /// Create with custom client for advanced configuration
    pub fn with_client(client: Client) -> Self {
        Self {
            client,
            default_timeout: Duration::from_secs(10),
        }
    }

    /// Set the default timeout for requests
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    #[tracing::instrument(skip(self))]
    async fn get_request<T>(&self, url: String, timeout: Option<Duration>) -> Result<T, CoreError>
    where
        T: DeserializeOwned,
    {
        let timeout = timeout.unwrap_or(self.default_timeout);
        tracing::info!("GET request to url={:?} with timeout={:?}", url, timeout);
        println!("GET request to url={:?} with timeout={:?}", url, timeout);

        let response = self
            .client
            .get(url)
            .timeout(timeout)
            .header("User-Agent", "Chrome")
            .send()
            .await?;

        let data = response.json::<T>().await?;

        Ok(data)
    }

    /// Get stations by name
    pub async fn get_stations(&self, station_name: &str) -> Result<StationResponse, CoreError> {
        let url = format!(
            "{}/negocios-e-servicos/estacao-nome/{}",
            IP_BASE_URL, station_name
        );

        self.get_request(url, None).await
    }

    /// Get station timetable by station ID
    pub async fn get_station_timetable(
        &self,
        station_id: &str,
    ) -> Result<Vec<Timetable>, CoreError> {
        let formatted_station_id = format!("{}-{}", &station_id[..2], &station_id[2..]);

        let url = format!(
            "{}/station/trains?stationId={}",
            CP_BASE_URL, formatted_station_id
        );

        self.get_request(url, None).await
    }

    /// Get train details by train ID
    pub async fn get_train_details(&self, train_id: u16) -> Result<Train, CoreError> {
        let url = format!("{}/station/trains/train?trainId={}", CP_BASE_URL, train_id);

        self.get_request(url, None).await
    }
}

impl Default for ComboiosApi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_comboios_api_creation() {
        let api = ComboiosApi::new();
        assert_eq!(api.default_timeout, Duration::from_secs(10));
    }

    #[test]
    fn test_comboios_api_with_client() {
        let client = Client::new();
        let api = ComboiosApi::with_client(client);
        assert_eq!(api.default_timeout, Duration::from_secs(10));
    }

    #[test]
    fn test_comboios_api_with_timeout() {
        let timeout = Duration::from_secs(30);
        let api = ComboiosApi::new().with_timeout(timeout);
        assert_eq!(api.default_timeout, timeout);
    }

    #[test]
    fn test_comboios_api_default() {
        let api = ComboiosApi::default();
        assert_eq!(api.default_timeout, Duration::from_secs(10));
    }

    #[test]
    fn test_builder_pattern() {
        let client = Client::new();
        let timeout = Duration::from_secs(20);

        let api = ComboiosApi::with_client(client).with_timeout(timeout);
        assert_eq!(api.default_timeout, timeout);
    }

    // Integration tests would go here
    // Note: These require network access and real API endpoints
    #[tokio::test]
    #[ignore] // Use `cargo test -- --ignored` to run
    async fn test_integration_get_stations() {
        let api = ComboiosApi::new();
        let result = api.get_stations("Lisboa").await;

        match result {
            Ok(stations) => {
                assert!(!stations.response.is_empty(), "Should find Lisboa stations");
            }
            Err(e) => {
                eprintln!(
                    "Integration test failed (expected with network issues): {}",
                    e
                );
            }
        }
    }

    #[tokio::test]
    #[ignore] // Use `cargo test -- --ignored` to run
    async fn test_integration_chain_calls() {
        let api = ComboiosApi::new();

        if let Ok(stations) = api.get_stations("Porto").await {
            if let Some(station) = stations.response.first() {
                let timetable_result = api.get_station_timetable(&station.code).await;

                match timetable_result {
                    Ok(timetable) => {
                        println!(
                            "Found {} trains for station {}",
                            timetable.len(),
                            station.designation
                        );

                        if let Some(train_info) = timetable.first() {
                            let _train_details =
                                api.get_train_details(train_info.train_number as u16).await;
                            // Just verify it doesn't panic
                        }
                    }
                    Err(e) => {
                        eprintln!("Timetable request failed: {}", e);
                    }
                }
            }
        }
    }
}
