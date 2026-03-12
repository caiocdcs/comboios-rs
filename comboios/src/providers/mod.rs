//! Data providers for different APIs
//!
//! This module provides a trait-based abstraction for different data sources,
//! allowing the library to work with multiple APIs (IP and CP) seamlessly.

use async_trait::async_trait;
use std::fmt::Debug;

use crate::{
    domain::{
        station::StationResponse,
        station_timetable::StationBoard,
        journey::{TrainJourney, JourneyStop},
    },
    error::CoreError,
};

/// Trait for data providers
#[async_trait]
pub trait DataProvider: Send + Sync + Debug {
    /// Provider name for logging/debugging
    fn name(&self) -> &'static str;
    
    /// Search stations by name
    async fn search_stations(&self, query: &str) -> Result<StationResponse, CoreError>;
    
    /// Get station timetable (departures/arrivals)
    async fn get_station_timetable(
        &self,
        station_id: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<Vec<StationBoard>, CoreError>;
    
    /// Get station board for next 12 hours (convenience method)
    async fn get_station_board_now(&self, station_id: &str) -> Result<Vec<StationBoard>, CoreError>;
    
    /// Get train details/journey (if available)
    /// Returns None if provider doesn't support this feature
    async fn get_train_journey(&self, _train_number: &str) -> Result<Option<TrainJourney>, CoreError> {
        Ok(None)
    }
    
    /// Get next N stops for a train
    /// Returns empty vec if provider doesn't support this feature
    async fn get_next_stops(
        &self,
        train_number: &str,
        count: usize,
    ) -> Result<Vec<JourneyStop>, CoreError> {
        let _ = (train_number, count);
        Ok(Vec::new())
    }
}

pub mod api_client;
pub mod ip_provider;
pub mod cp_provider;
pub mod unified;
pub mod id_mapping;

pub use api_client::ComboiosClient;
pub use ip_provider::IpProvider;
pub use cp_provider::{CpConfig, CpProvider};
pub use unified::UnifiedProvider;
pub use id_mapping::{to_cp_id, to_ip_id, normalize_station_id};
