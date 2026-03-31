//! Comboios de Portugal (CP) API Gateway Provider
//!
//! This provider uses the CP API Gateway for enhanced features like
//! train journeys and detailed station information. Requires authentication.

use async_trait::async_trait;
use std::time::Duration;

use crate::{
    domain::{
        journey::{JourneyResponse, JourneyStop, TrainJourney},
        station::StationResponse,
        station_timetable::StationBoard,
    },
    error::CoreError,
    providers::{to_cp_id, DataProvider},
};

const CP_BASE_URL: &str = "https://api-gateway.cp.pt/cp";

/// CP API Provider with authentication
#[derive(Debug, Clone)]
pub struct CpProvider {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    connect_id: String,
    connect_secret: String,
}

impl CpProvider {
    /// Create new CP provider with authentication credentials
    /// 
    /// # Arguments
    /// * `api_key` - The x-api-key header value
    /// * `connect_id` - The x-cp-connect-id header value
    /// * `connect_secret` - The x-cp-connect-secret header value
    pub fn new(
        api_key: impl Into<String>,
        connect_id: impl Into<String>,
        connect_secret: impl Into<String>,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: CP_BASE_URL.to_string(),
            api_key: api_key.into(),
            connect_id: connect_id.into(),
            connect_secret: connect_secret.into(),
        }
    }
    
    /// Build authenticated request with all required headers
    fn build_request(&self, method: reqwest::Method, url: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, url)
            .timeout(Duration::from_secs(30))
            .header("x-api-key", &self.api_key)
            .header("x-cp-connect-id", &self.connect_id)
            .header("x-cp-connect-secret", &self.connect_secret)
            .header("Accept", "application/json")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .header("Origin", "https://www.cp.pt")
            .header("Referer", "https://www.cp.pt/")
    }
    
    /// Get station info endpoint URL
    #[allow(dead_code)]
    fn station_info_url(&self, station_id: &str) -> String {
        let cp_id = to_cp_id(station_id);
        format!("{}/services/stations-api/stations/infos/{}", self.base_url, cp_id)
    }
    
    /// Get timetable endpoint URL
    fn timetable_url(&self, station_id: &str, date: &str) -> String {
        let cp_id = to_cp_id(station_id);
        format!(
            "{}/services/travel-api/stations/{}/timetable/{}",
            self.base_url, cp_id, date
        )
    }
    
    /// Get train journey endpoint URL
    fn train_journey_url(&self, train_number: &str, date: &str) -> String {
        format!(
            "{}/services/travel-api/trains/{}/journey/{}",
            self.base_url, train_number, date
        )
    }
}

#[async_trait]
impl DataProvider for CpProvider {
    fn name(&self) -> &'static str {
        "Comboios de Portugal API"
    }
    
    async fn search_stations(&self, _query: &str) -> Result<StationResponse, CoreError> {
        // CP API doesn't have a search endpoint, fallback to returning empty
        // In unified mode, this will fallback to IP provider
        Err(CoreError::InvalidInput(
            "CP provider doesn't support station search. Use IP provider.".into(),
        ))
    }
    
    async fn get_station_timetable(
        &self,
        station_id: &str,
        start_time: &str,
        _end_time: &str,
    ) -> Result<Vec<StationBoard>, CoreError> {
        // CP API uses date-based queries, parse from start_time
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let date = start_time.split_whitespace().next().unwrap_or(&today);
        let start = start_time.split_whitespace().nth(1).unwrap_or("00:00");
        
        let url = self.timetable_url(station_id, date);
        let url = format!("{}?start={}", url, start);
        
        let response = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            
            // If auth failed, give clear error
            if status == 401 {
                return Err(CoreError::InvalidInput(
                    "CP API authentication failed. Check credentials.".into(),
                ));
            }
            
            return Err(CoreError::ApiError { status, message: text });
        }
        
        // TODO: Parse CP response format (different from IP)
        // For now return empty, will implement based on actual response
        let _text = response.text().await?;
        Ok(Vec::new())
    }
    
    async fn get_station_board_now(&self, station_id: &str) -> Result<Vec<StationBoard>, CoreError> {
        use chrono::Local;
        
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let start = now.format("%H:%M").to_string();
        let end = "23:59";
        
        // Note: CP API might need different time format
        self.get_station_timetable(station_id, &format!("{} {}", date, start), &format!("{} {}", date, end)).await
    }
    
    async fn get_train_journey(&self, train_number: &str) -> Result<Option<TrainJourney>, CoreError> {
        use chrono::Local;
        
        let date = Local::now().format("%Y-%m-%d").to_string();
        let url = self.train_journey_url(train_number, &date);
        
        let response = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await?;
        
        if response.status().as_u16() == 404 {
            // Train not found for today
            return Ok(None);
        }
        
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(CoreError::ApiError { status, message: text });
        }
        
        let _data: JourneyResponse = response.json().await?;
        // TODO: Map CP response to our TrainJourney type
        Ok(None)
    }
    
    async fn get_next_stops(
        &self,
        train_number: &str,
        count: usize,
    ) -> Result<Vec<JourneyStop>, CoreError> {
        if let Some(journey) = self.get_train_journey(train_number).await? {
            let stops = journey.upcoming_stops(count);
            // Convert refs to owned
            Ok(stops.into_iter().cloned().collect())
        } else {
            Ok(Vec::new())
        }
    }
}

/// Configuration for CP provider
#[derive(Debug, Clone)]
pub struct CpConfig {
    pub api_key: String,
    pub connect_id: String,
    pub connect_secret: String,
}

impl CpConfig {
    /// Load from environment variables
    pub fn from_env() -> Option<Self> {
        use std::env;
        
        let api_key = env::var("CP_API_KEY").ok()?;
        let connect_id = env::var("CP_CONNECT_ID").ok()?;
        let connect_secret = env::var("CP_CONNECT_SECRET").ok()?;
        
        Some(Self {
            api_key,
            connect_id,
            connect_secret,
        })
    }
}
