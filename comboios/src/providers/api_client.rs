//! High-level API client using the provider pattern
//!
//! This is the new recommended way to use the API, supporting both IP and CP providers.

use std::sync::Arc;

use crate::{
    domain::{
        journey::{JourneyStop, TrainJourney},
        station::StationResponse,
        station_timetable::StationBoard,
    },
    error::CoreError,
    providers::{CpConfig, CpProvider, DataProvider, IpProvider, UnifiedProvider},
};

/// Modern Comboios API client with dual-provider support
///
/// This client supports both Infraestruturas de Portugal (IP) API
/// and Comboios de Portugal (CP) API Gateway with automatic fallback.
///
/// # Examples
///
/// Basic usage with IP provider only:
/// ```no_run
/// use comboios::providers::ComboiosClient;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ComboiosClient::new();
/// let stations = client.search_stations("Lisboa").await?;
/// # Ok(())
/// # }
/// ```
///
/// With CP provider for journey tracking:
/// ```ignore
/// use comboios::providers::{ComboiosClient, CpConfig};
///
/// let cp_config = CpConfig::from_env().expect("CP credentials not set");
/// let client = ComboiosClient::with_cp(cp_config);
///
/// let journey = client.get_train_journey("722").await?;
/// ```
#[derive(Debug, Clone)]
pub struct ComboiosClient {
    provider: Arc<dyn DataProvider>,
}

impl ComboiosClient {
    /// Create a new client with IP provider only (no authentication required)
    pub fn new() -> Self {
        Self::with_ip()
    }
    
    /// Create with IP provider
    pub fn with_ip() -> Self {
        Self {
            provider: Arc::new(IpProvider::new()),
        }
    }
    
    /// Create with CP provider (requires authentication)
    pub fn with_cp(config: CpConfig) -> Self {
        Self {
            provider: Arc::new(CpProvider::new(
                config.api_key,
                config.connect_id,
                config.connect_secret,
            )),
        }
    }
    
    /// Create with unified provider (IP primary, CP fallback for journeys)
    pub fn with_unified(ip: IpProvider, cp: CpProvider) -> Self {
        let unified = UnifiedProvider::new(Box::new(ip))
            .with_fallback(Box::new(cp));
        
        Self {
            provider: Arc::new(unified),
        }
    }
    
    /// Search stations by name
    pub async fn search_stations(&self, query: &str) -> Result<StationResponse, CoreError> {
        self.provider.search_stations(query).await
    }
    
    /// Get station timetable for a time range
    pub async fn get_station_timetable(
        &self,
        station_id: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<Vec<StationBoard>, CoreError> {
        self.provider.get_station_timetable(station_id, start_time, end_time).await
    }
    
    /// Get station board for next 12 hours (convenience method)
    pub async fn get_station_board_now(&self, station_id: &str) -> Result<Vec<StationBoard>, CoreError> {
        self.provider.get_station_board_now(station_id).await
    }
    
    /// Get train journey (requires CP provider)
    pub async fn get_train_journey(&self, train_number: &str) -> Result<Option<TrainJourney>, CoreError> {
        self.provider.get_train_journey(train_number).await
    }
    
    /// Get next N stops for a train (requires CP provider)
    pub async fn get_next_stops(&self, train_number: &str, count: usize) -> Result<Vec<JourneyStop>, CoreError> {
        self.provider.get_next_stops(train_number, count).await
    }
    
    /// Check if a train is currently at a specific station
    pub async fn is_train_at_station(&self, train_number: &str, station_id: &str) -> Result<bool, CoreError> {
        if let Some(journey) = self.get_train_journey(train_number).await? {
            let current = journey.current_stop();
            Ok(current.map(|s| s.station.code == station_id).unwrap_or(false))
        } else {
            Ok(false)
        }
    }
}

impl Default for ComboiosClient {
    fn default() -> Self {
        Self::new()
    }
}
