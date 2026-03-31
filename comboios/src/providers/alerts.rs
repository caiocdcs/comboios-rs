//! Alert providers for service disruption information
//!
//! This module provides implementations for fetching service alerts
//! from various sources including Infraestruturas de Portugal.

//! Alert providers for service disruption information
//!
//! This module provides implementations for fetching service alerts
//! from various sources including Infraestruturas de Portugal.

use async_trait::async_trait;
use chrono::Utc;
use scraper::{Html, Selector};

use crate::{
    domain::alert::{ServiceAlert, AlertSeverity, AlertCategory, AlertSource},
    error::CoreError,
    providers::DataProvider,
};

/// Provider for Infraestruturas de Portugal service alerts
#[derive(Debug, Clone)]
pub struct IpAlertsProvider {
    client: reqwest::Client,
    base_url: String,
}

impl Default for IpAlertsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl IpAlertsProvider {
    /// Create a new IP alerts provider with default settings
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://servicos.infraestruturasdeportugal.pt".to_string(),
        }
    }
    
    /// Create with custom base URL (useful for testing)
    pub fn with_url(base_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }
    
    /// Create with custom client
    pub fn with_client(client: reqwest::Client, base_url: Option<&str>) -> Self {
        Self {
            client,
            base_url: base_url.unwrap_or("https://servicos.infraestruturasdeportugal.pt").to_string(),
        }
    }
    
    /// Get the alerts page URL
    fn alerts_url(&self) -> String {
        format!("{}/pt-pt/alertas", self.base_url)
    }
    
    /// Parse alerts from HTML content
    fn parse_alerts(&self, html: &str) -> Result<Vec<ServiceAlert>, CoreError> {
        let document = Html::parse_document(html);
        let alert_selector = Selector::parse(".alertas").map_err(|e| {
            CoreError::InvalidInput(format!("Failed to parse alert selector: {}", e))
        })?;
        
        let title_selector = Selector::parse(".tituloalertas a").map_err(|e| {
            CoreError::InvalidInput(format!("Failed to parse title selector: {}", e))
        })?;
        
        let mut alerts = Vec::new();
        let now = Utc::now();
        
        for element in document.select(&alert_selector) {
            // Extract title
            let title_element = element.select(&title_selector).next();
            if let Some(title_el) = title_element {
                let title = title_el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                let href = title_el.value().attr("href").unwrap_or("").to_string();
                
                if !title.is_empty() {
                    let alert = ServiceAlert {
                        id: format!("ip-{}", href.replace("/", "-").replace(".html", "")),
                        title,
                        description: "Consulte mais detalhes no site oficial.".to_string(),
                        severity: AlertSeverity::Warning, // Default to warning for infrastructure alerts
                        category: AlertCategory::Infrastructure,
                        affected_lines: Vec::new(), // Would need more parsing to extract this
                        affected_stations: Vec::new(), // Would need more parsing to extract this
                        start_time: Some(now),
                        end_time: None,
                        last_updated: now,
                        url: if href.starts_with("http") {
                            Some(href)
                        } else {
                            Some(format!("{}{}", self.base_url, href))
                        },
                        source: AlertSource::InfraestruturasPortugal,
                    };
                    alerts.push(alert);
                }
            }
        }
        
        Ok(alerts)
    }
}

#[async_trait]
impl DataProvider for IpAlertsProvider {
    fn name(&self) -> &'static str {
        "Infraestruturas de Portugal Alerts"
    }
    
    /// Search stations - not supported by alerts provider
    async fn search_stations(&self, _query: &str) -> Result<crate::domain::station::StationResponse, CoreError> {
        Err(CoreError::InvalidInput(
            "Alerts provider doesn't support station search".into(),
        ))
    }
    
    /// Get station timetable - not supported by alerts provider
    async fn get_station_timetable(
        &self,
        _station_id: &str,
        _start_time: &str,
        _end_time: &str,
    ) -> Result<Vec<crate::domain::station_timetable::StationBoard>, CoreError> {
        Err(CoreError::InvalidInput(
            "Alerts provider doesn't support station timetables".into(),
        ))
    }
    
    /// Get current station board - not supported by alerts provider
    async fn get_station_board_now(&self, _station_id: &str) -> Result<Vec<crate::domain::station_timetable::StationBoard>, CoreError> {
        Err(CoreError::InvalidInput(
            "Alerts provider doesn't support station boards".into(),
        ))
    }
    
    /// Get service alerts
    async fn get_service_alerts(&self) -> Result<Vec<ServiceAlert>, CoreError> {
        let url = self.alerts_url();
        
        let response = self
            .client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (compatible; Comboios-RS/1.0)")
            .send()
            .await
            .map_err(CoreError::NetworkError)?;
        
        if !response.status().is_success() {
            return Err(CoreError::ApiError {
                status: response.status().as_u16(),
                message: "Failed to fetch IP alerts".to_string(),
            });
        }
        
        let html = response
            .text()
            .await
            .map_err(CoreError::NetworkError)?;
        
        self.parse_alerts(&html)
    }
}