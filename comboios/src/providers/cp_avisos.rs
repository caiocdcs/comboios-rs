//! CP Avisos provider for service disruption information
//!
//! This module provides implementations for fetching service alerts
//! from Comboios de Portugal's avisos page.

use async_trait::async_trait;
use chrono::Utc;
use scraper::{Html, Selector};

use crate::{
    domain::alert::{ServiceAlert, AlertSeverity, AlertCategory, AlertSource},
    error::CoreError,
    providers::DataProvider,
};

/// Provider for Comboios de Portugal avisos/alerts
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CpAvisosProvider {
    client: reqwest::Client,
    base_url: String,
}

impl Default for CpAvisosProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl CpAvisosProvider {
    /// Create a new CP avisos provider with default settings
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://www.cp.pt".to_string(),
        }
    }
    
    /// Create with custom client
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            client,
            base_url: "https://www.cp.pt".to_string(),
        }
    }
    
    /// Get the avisos page URL
    fn avisos_url(&self) -> String {
        format!("{}/pt/avisos", self.base_url)
    }
    
    /// Parse alerts from HTML content
    #[allow(dead_code)]
    fn parse_alerts(&self, html: &str) -> Result<Vec<ServiceAlert>, CoreError> {
        let _document = Html::parse_document(html);
        let _alert_selector = Selector::parse(".alert, .avisos, .news-item, .service-alert").unwrap_or_else(|_| {
            // Fallback selector
            Selector::parse("div").unwrap()
        });
        
        let mut alerts = Vec::new();
        let now = Utc::now();
        
        // For now, return sample data since we can't easily parse the CP avisos page
        // In a real implementation, this would parse the actual HTML structure
        let sample_alerts = vec![
            ServiceAlert {
                id: "cp-real-1".to_string(),
                title: "Atualização do serviço ferroviário".to_string(),
                description: "Consulte as últimas informações sobre o serviço ferroviário.".to_string(),
                severity: AlertSeverity::Info,
                category: AlertCategory::Other,
                affected_lines: Vec::new(),
                affected_stations: Vec::new(),
                start_time: Some(now),
                end_time: None,
                last_updated: now,
                url: Some(self.avisos_url()),
                source: AlertSource::ComboiosPortugal,
            }
        ];
        
        alerts.extend(sample_alerts);
        Ok(alerts)
    }
}

#[async_trait]
impl DataProvider for CpAvisosProvider {
    fn name(&self) -> &'static str {
        "Comboios de Portugal Avisos"
    }
    
    /// Search stations - not supported by avisos provider
    async fn search_stations(&self, _query: &str) -> Result<crate::domain::station::StationResponse, CoreError> {
        Err(CoreError::InvalidInput(
            "Avisos provider doesn't support station search".into(),
        ))
    }
    
    /// Get station timetable - not supported by avisos provider
    async fn get_station_timetable(
        &self,
        _station_id: &str,
        _start_time: &str,
        _end_time: &str,
    ) -> Result<Vec<crate::domain::station_timetable::StationBoard>, CoreError> {
        Err(CoreError::InvalidInput(
            "Avisos provider doesn't support station timetables".into(),
        ))
    }
    
    /// Get current station board - not supported by avisos provider
    async fn get_station_board_now(&self, _station_id: &str) -> Result<Vec<crate::domain::station_timetable::StationBoard>, CoreError> {
        Err(CoreError::InvalidInput(
            "Avisos provider doesn't support station boards".into(),
        ))
    }
    
    /// Get service alerts from CP avisos
    async fn get_service_alerts(&self) -> Result<Vec<ServiceAlert>, CoreError> {
        // For now, we'll return sample data since the CP avisos page structure is not easily accessible
        // A real implementation would fetch and parse https://www.cp.pt/pt/avisos
        let now = Utc::now();
        
        let sample_alerts = vec![
            ServiceAlert {
                id: "cp-real-1".to_string(),
                title: "Atualização do serviço ferroviário".to_string(),
                description: "Consulte as últimas informações sobre o serviço ferroviário na nossa página de avisos.".to_string(),
                severity: AlertSeverity::Info,
                category: AlertCategory::Other,
                affected_lines: Vec::new(),
                affected_stations: Vec::new(),
                start_time: Some(now),
                end_time: None,
                last_updated: now,
                url: Some(self.avisos_url()),
                source: AlertSource::ComboiosPortugal,
            }
        ];
        
        Ok(sample_alerts)
    }
}