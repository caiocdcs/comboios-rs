use std::time::Duration;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{
    domain::{
        station::StationResponse,
        station_timetable::{StationBoard, StationBoardResponse},
        train::Train,
    },
    error::CoreError,
};

const CP_BASE_URL: &str = "https://www.cp.pt/sites/spring";
const IP_BASE_URL: &str = "https://www.infraestruturasdeportugal.pt";

/// Configuration for retry behavior
#[derive(Debug, Clone, Copy)]
pub struct RetryConfig {
    /// Maximum number of retry attempts (0 = disabled)
    pub max_retries: u32,
    /// Base delay between retries
    pub base_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Whether to retry on server errors (5xx)
    pub retry_on_server_errors: bool,
    /// Whether to retry on network errors
    pub retry_on_network_errors: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(10),
            retry_on_server_errors: true,
            retry_on_network_errors: true,
        }
    }
}

impl RetryConfig {
    /// Create a disabled retry configuration
    pub fn disabled() -> Self {
        Self {
            max_retries: 0,
            base_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
            retry_on_server_errors: false,
            retry_on_network_errors: false,
        }
    }

    /// Calculate delay for a specific retry attempt using exponential backoff
    fn calculate_delay(&self, attempt: u32) -> Duration {
        let exponential = self.base_delay * 2_u32.pow(attempt);
        std::cmp::min(exponential, self.max_delay)
    }
}

/// Configuration for API base URLs
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub cp_base_url: String,
    pub ip_base_url: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            cp_base_url: CP_BASE_URL.to_string(),
            ip_base_url: IP_BASE_URL.to_string(),
        }
    }
}

/// Main API client for Comboios operations
///
/// This client provides access to Portuguese train (Comboios de Portugal) APIs
/// for searching stations, retrieving timetables, and getting train details.
///
/// # Examples
///
/// Basic usage:
/// ```ignore
/// use comboios::ComboiosApi;
///
/// let api = ComboiosApi::new();
/// let stations = api.get_stations("Lisboa").await?;
/// ```
///
/// With custom configuration:
/// ```ignore
/// use comboios::{ComboiosApi, RetryConfig};
/// use std::time::Duration;
///
/// let api = ComboiosApi::new()
///     .with_timeout(Duration::from_secs(30))
///     .with_retry(RetryConfig::default());
/// ```
#[derive(Debug, Clone)]
pub struct ComboiosApi {
    client: Client,
    default_timeout: Duration,
    config: ApiConfig,
    retry_config: RetryConfig,
}

impl ComboiosApi {
    /// Create a new instance with default reqwest client
    ///
    /// # Examples
    /// ```
    /// use comboios::ComboiosApi;
    ///
    /// let api = ComboiosApi::new();
    /// ```
    pub fn new() -> Self {
        Self::with_config(ApiConfig::default())
    }

    /// Create with custom client for advanced configuration
    ///
    /// # Examples
    /// ```ignore
    /// use comboios::ComboiosApi;
    /// use reqwest::Client;
    ///
    /// let client = Client::builder()
    ///     .timeout(Duration::from_secs(60))
    ///     .build()?;
    ///
    /// let api = ComboiosApi::with_client(client);
    /// ```
    pub fn with_client(client: Client) -> Self {
        Self {
            client,
            default_timeout: Duration::from_secs(10),
            config: ApiConfig::default(),
            retry_config: RetryConfig::default(),
        }
    }

    /// Create with custom API configuration (useful for testing)
    pub fn with_config(config: ApiConfig) -> Self {
        Self {
            client: Client::new(),
            default_timeout: Duration::from_secs(10),
            config,
            retry_config: RetryConfig::default(),
        }
    }

    /// Create with both custom client and config
    pub fn with_client_and_config(client: Client, config: ApiConfig) -> Self {
        Self {
            client,
            default_timeout: Duration::from_secs(10),
            config,
            retry_config: RetryConfig::default(),
        }
    }

    /// Set the default timeout for requests
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Set retry configuration
    pub fn with_retry(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = retry_config;
        self
    }

    /// Check if error should trigger a retry
    fn should_retry(&self, error: &CoreError) -> bool {
        match error {
            CoreError::NetworkError(_) => self.retry_config.retry_on_network_errors,
            CoreError::ApiError { status, .. } => {
                self.retry_config.retry_on_server_errors && *status >= 500 && *status < 600
            }
            _ => false,
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    async fn get_request<T>(&self, url: String, timeout: Option<Duration>) -> Result<T, CoreError>
    where
        T: DeserializeOwned,
    {
        let timeout = timeout.unwrap_or(self.default_timeout);
        let mut last_error = None;

        for attempt in 0..=self.retry_config.max_retries {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "GET request attempt {} to url={:?} with timeout={:?}",
                attempt + 1,
                url,
                timeout
            );

            match self.execute_request::<T>(&url, timeout).await {
                Ok(data) => return Ok(data),
                Err(err) => {
                    let should_retry = self.should_retry(&err);
                    last_error = Some(err);

                    if !should_retry || attempt >= self.retry_config.max_retries {
                        break;
                    }

                    let delay = self.retry_config.calculate_delay(attempt);
                    #[cfg(feature = "tracing")]
                    tracing::debug!("Retrying after {:?} delay", delay);
                    tokio::time::sleep(delay).await;
                }
            }
        }

        Err(last_error.unwrap())
    }

    async fn execute_request<T>(&self, url: &str, timeout: Duration) -> Result<T, CoreError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(url)
            .timeout(timeout)
            .header("User-Agent", "Chrome")
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(CoreError::ApiError {
                status: status.as_u16(),
                message: text,
            });
        }

        let data = response.json::<T>().await?;
        Ok(data)
    }

    /// Get stations by name
    pub async fn get_stations(&self, query: &str) -> Result<StationResponse, CoreError> {
        let url = format!(
            "{}/negocios-e-servicos/estacao-nome/{}",
            self.config.ip_base_url, query
        );

        self.get_request(url, None).await
    }

    /// Get station timetable (departures and arrivals) by station ID
    ///
    /// Uses the Infraestruturas de Portugal API which provides real-time
    /// station board information including delays and service types.
    ///
    /// # Arguments
    /// * `station_id` - The station NodeID (e.g., "9431039" for Lisboa-Oriente)
    /// * `start_time` - Start time for the query (format: "YYYY-MM-DD HH:MM")
    /// * `end_time` - End time for the query (format: "YYYY-MM-DD HH:MM")
    ///
    /// # Example
    /// ```ignore
    /// use comboios::ComboiosApi;
    ///
    /// let api = ComboiosApi::new();
    /// let boards = api.get_station_timetable("9431039", "2026-03-11 06:00", "2026-03-11 20:00").await?;
    /// ```
    pub async fn get_station_timetable(
        &self,
        station_id: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<Vec<StationBoard>, CoreError> {
        // Validate station_id to prevent panics
        if station_id.is_empty() {
            return Err(CoreError::InvalidInput(
                "station_id cannot be empty".into(),
            ));
        }

        // Services to include: all train types (with spaces as required by API)
        let services = "INTERNACIONAL, ALFA, IC, IR, REGIONAL, URB|SUBUR, ESPECIAL";

        let url = format!(
            "{}/negocios-e-servicos/partidas-chegadas/{}/{}/{}/{}",
            self.config.ip_base_url,
            station_id,
            urlencoding::encode(start_time),
            urlencoding::encode(end_time),
            services
        );

        let response: StationBoardResponse = self.get_request(url, None).await?;
        Ok(response.response)
    }

    /// Get train details by train ID
    ///
    /// # Deprecated
    /// This method is currently unavailable as the CP API endpoint has been removed.
    /// It may be restored in a future version if a replacement endpoint is found.
    #[deprecated(since = "0.2.0", note = "CP API endpoint no longer available")]
    pub async fn get_train_details(&self, _train_id: u16) -> Result<Train, CoreError> {
        Err(CoreError::InvalidInput(
            "Train details API is no longer available. Use get_station_timetable instead.".into(),
        ))
    }

    /// Search for stations using a query builder
    ///
    /// # Example
    /// ```ignore
    /// use comboios::{ComboiosApi, StationQuery};
    ///
    /// let api = ComboiosApi::new();
    /// let query = StationQuery::new()
    ///     .name("Lisboa")
    ///     .limit(10);
    ///
    /// let stations = api.find_stations(query).await?;
    /// ```
    pub async fn find_stations(
        &self,
        query: crate::query_builder::StationQuery,
    ) -> Result<StationResponse, CoreError> {
        let name = query.build();
        let mut response = self.get_stations(&name).await?;

        // Apply limit if specified
        if let Some(limit) = query.get_limit() {
            response.response.truncate(limit);
        }

        Ok(response)
    }

    /// Get station timetable with optional filtering
    ///
    /// # Example
    /// ```ignore
    /// use comboios::ComboiosApi;
    ///
    /// let api = ComboiosApi::new();
    /// let boards = api.get_station_timetable("9431039", "2026-03-11 06:00", "2026-03-11 20:00").await?;
    ///
    /// // Filter by service type
    /// let ic_trains: Vec<_> = boards.iter()
    ///     .flat_map(|b| &b.trains)
    ///     .filter(|t| t.service_type == "IC")
    ///     .collect();
    /// ```
    pub async fn get_station_timetable_filtered(
        &self,
        station_id: &str,
        start_time: &str,
        end_time: &str,
        service_type: Option<&str>,
    ) -> Result<Vec<StationBoard>, CoreError> {
        let mut boards = self.get_station_timetable(station_id, start_time, end_time).await?;

        if let Some(service) = service_type {
            for board in &mut boards {
                board.trains.retain(|t| t.service_type == service);
            }
            // Remove empty boards
            boards.retain(|b| !b.trains.is_empty());
        }

        Ok(boards)
    }

    /// Get current station board for the next 12 hours
    ///
    /// Convenience method that automatically sets time range
    pub async fn get_station_board_now(
        &self,
        station_id: &str,
    ) -> Result<Vec<StationBoard>, CoreError> {
        use chrono::{Duration, Local};

        let now = Local::now();
        let start = now.format("%Y-%m-%d %H:%M").to_string();
        let end = (now + Duration::hours(12)).format("%Y-%m-%d %H:%M").to_string();

        self.get_station_timetable(station_id, &start, &end).await
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

    #[test]
    fn test_api_config_default() {
        let config = ApiConfig::default();
        assert_eq!(config.cp_base_url, CP_BASE_URL);
        assert_eq!(config.ip_base_url, IP_BASE_URL);
    }

    #[test]
    fn test_comboios_api_with_config() {
        let config = ApiConfig {
            cp_base_url: "http://localhost:8080".to_string(),
            ip_base_url: "http://localhost:8081".to_string(),
        };
        let api = ComboiosApi::with_config(config);
        assert_eq!(api.default_timeout, Duration::from_secs(10));
    }
}
