//! Unified provider with fallback support

use async_trait::async_trait;

use crate::{
    domain::{
        journey::{JourneyStop, TrainJourney},
        station::StationResponse,
        station_timetable::StationBoard,
    },
    error::CoreError,
    providers::DataProvider,
};

/// Unified provider with fallback support
#[derive(Debug)]
pub struct UnifiedProvider {
    primary: Box<dyn DataProvider>,
    fallback: Option<Box<dyn DataProvider>>,
    enable_fallback: bool,
}

impl UnifiedProvider {
    /// Create with primary provider
    pub fn new(primary: Box<dyn DataProvider>) -> Self {
        Self {
            primary,
            fallback: None,
            enable_fallback: false,
        }
    }
    
    /// Add fallback provider
    pub fn with_fallback(mut self, fallback: Box<dyn DataProvider>) -> Self {
        self.fallback = Some(fallback);
        self.enable_fallback = true;
        self
    }
    
    /// Check if error should trigger fallback
    fn should_fallback(&self, error: &CoreError) -> bool {
        match error {
            CoreError::NetworkError(_) => true,
            CoreError::ApiError { status, .. } => *status >= 500,
            _ => false,
        }
    }
}

#[async_trait]
impl DataProvider for UnifiedProvider {
    fn name(&self) -> &'static str {
        "Unified (Multi-Provider)"
    }
    
    async fn search_stations(&self, query: &str) -> Result<StationResponse, CoreError> {
        match self.primary.search_stations(query).await {
            Ok(result) => Ok(result),
            Err(e) if self.enable_fallback => {
                if let Some(ref fallback) = self.fallback {
                    fallback.search_stations(query).await
                } else {
                    Err(e)
                }
            }
            Err(e) => Err(e),
        }
    }
    
    async fn get_station_timetable(
        &self,
        station_id: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<Vec<StationBoard>, CoreError> {
        match self.primary.get_station_timetable(station_id, start_time, end_time).await {
            Ok(result) => Ok(result),
            Err(e) if self.enable_fallback && self.should_fallback(&e) => {
                if let Some(ref fallback) = self.fallback {
                    fallback.get_station_timetable(station_id, start_time, end_time).await
                } else {
                    Err(e)
                }
            }
            Err(e) => Err(e),
        }
    }
    
    async fn get_station_board_now(&self, station_id: &str) -> Result<Vec<StationBoard>, CoreError> {
        match self.primary.get_station_board_now(station_id).await {
            Ok(result) => Ok(result),
            Err(e) if self.enable_fallback && self.should_fallback(&e) => {
                if let Some(ref fallback) = self.fallback {
                    fallback.get_station_board_now(station_id).await
                } else {
                    Err(e)
                }
            }
            Err(e) => Err(e),
        }
    }
    
    async fn get_train_journey(&self, train_number: &str) -> Result<Option<TrainJourney>, CoreError> {
        // For journey data, try CP first if available, don't fallback
        self.primary.get_train_journey(train_number).await
    }
    
    async fn get_next_stops(&self, train_number: &str, count: usize) -> Result<Vec<JourneyStop>, CoreError> {
        self.primary.get_next_stops(train_number, count).await
    }
}
