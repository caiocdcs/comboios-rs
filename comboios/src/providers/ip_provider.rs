//! Infraestruturas de Portugal (IP) API Provider
//!
//! This provider uses the public IP API for station search and timetables.
//! It does not require authentication and provides real-time delay information.

use async_trait::async_trait;
use std::time::Duration;

use crate::{
    domain::{
        station::StationResponse,
        station_timetable::{StationBoard, StationBoardResponse},
        train_journey::IpTrainJourneyResponse,
        journey::TrainJourney,
    },
    error::CoreError,
    providers::DataProvider,
};

const IP_BASE_URL: &str = "https://www.infraestruturasdeportugal.pt";

/// IP API Provider
#[derive(Debug, Clone)]
pub struct IpProvider {
    client: reqwest::Client,
    base_url: String,
}

impl IpProvider {
    /// Create a new IP provider with default settings
    pub fn new() -> Self {
        Self::with_url(IP_BASE_URL)
    }
    
    /// Create with custom base URL (useful for testing)
    pub fn with_url(base_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }
    
    /// Create with custom client
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            client,
            base_url: IP_BASE_URL.to_string(),
        }
    }
    
    /// Build station search URL
    fn station_search_url(&self, query: &str) -> String {
        format!("{}/negocios-e-servicos/estacao-nome/{}", self.base_url, query)
    }
    
    /// Build timetable URL
    fn timetable_url(
        &self,
        station_id: &str,
        start_time: &str,
        end_time: &str,
    ) -> String {
        let services = "INTERNACIONAL, ALFA, IC, IR, REGIONAL, URB|SUBUR, ESPECIAL";
        format!(
            "{}/negocios-e-servicos/partidas-chegadas/{}/{}/{}/{}",
            self.base_url,
            station_id,
            urlencoding::encode(start_time),
            urlencoding::encode(end_time),
            urlencoding::encode(services)
        )
    }
    
    /// Build train journey URL
    fn train_journey_url(&self, train_number: &str, date: &str) -> String {
        format!(
            "{}/negocios-e-servicos/horarios-ncombio/{}/{}",
            self.base_url,
            train_number,
            urlencoding::encode(date)
        )
    }
}

#[async_trait]
impl DataProvider for IpProvider {
    fn name(&self) -> &'static str {
        "Infraestruturas de Portugal"
    }
    
    async fn search_stations(&self, query: &str) -> Result<StationResponse, CoreError> {
        if query.is_empty() {
            return Err(CoreError::InvalidInput("Query cannot be empty".into()));
        }
        
        let url = self.station_search_url(query);
        
        let response = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(10))
            .header("User-Agent", "Chrome")
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(CoreError::ApiError { status, message: text });
        }
        
        let data: StationResponse = response.json().await?;
        Ok(data)
    }
    
    async fn get_station_timetable(
        &self,
        station_id: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<Vec<StationBoard>, CoreError> {
        if station_id.is_empty() {
            return Err(CoreError::InvalidInput(
                "station_id cannot be empty".into(),
            ));
        }
        
        let url = self.timetable_url(station_id, start_time, end_time);
        
        let response = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(30))
            .header("User-Agent", "Chrome")
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(CoreError::ApiError { status, message: text });
        }
        
        let data: StationBoardResponse = response.json().await?;
        Ok(data.response)
    }
    
    async fn get_station_board_now(&self, station_id: &str) -> Result<Vec<StationBoard>, CoreError> {
        use chrono::{Duration, Local};
        
        let now = Local::now();
        let start = now.format("%Y-%m-%d %H:%M").to_string();
        let end = (now + Duration::hours(12)).format("%Y-%m-%d %H:%M").to_string();
        
        self.get_station_timetable(station_id, &start, &end).await
    }
    
    async fn get_train_journey(&self, train_number: &str) -> Result<Option<TrainJourney>, CoreError> {
        use chrono::Local;
        
        let date = Local::now().format("%Y-%m-%d").to_string();
        let url = self.train_journey_url(train_number, &date);
        
        let response = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(30))
            .header("User-Agent", "Chrome")
            .send()
            .await?;
        
        if response.status().as_u16() == 404 {
            return Ok(None);
        }
        
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(CoreError::ApiError { status, message: text });
        }
        
        let data: IpTrainJourneyResponse = response.json().await?;
        Ok(Some(data.to_train_journey(train_number)))
    }
}

impl Default for IpProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_url_building() {
        let provider = IpProvider::new();
        
        let search_url = provider.station_search_url("Lisboa");
        assert!(search_url.contains("estacao-nome/Lisboa"));
        
        let timetable_url = provider.timetable_url("9431039", "2026-03-11 10:00", "2026-03-11 20:00");
        assert!(timetable_url.contains("partidas-chegadas/9431039"));
    }
}
