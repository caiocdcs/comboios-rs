//! Unified alert system for combining alerts from multiple sources
//!
//! This module provides a system for fetching and combining service alerts
//! from multiple providers including Infraestruturas de Portugal and Comboios de Portugal.

use crate::{
    domain::alert::ServiceAlert,
    error::CoreError,
    providers::{DataProvider, IpAlertsProvider, CpAvisosProvider},
};

/// Unified alert system that combines alerts from multiple sources
pub struct UnifiedAlertSystem {
    ip_provider: IpAlertsProvider,
    cp_provider: CpAvisosProvider,
}

impl UnifiedAlertSystem {
    /// Create a new unified alert system with default providers
    pub fn new() -> Self {
        Self {
            ip_provider: IpAlertsProvider::new(),
            cp_provider: CpAvisosProvider::new(),
        }
    }
    
    /// Create with custom providers
    pub fn with_providers(ip_provider: IpAlertsProvider, cp_provider: CpAvisosProvider) -> Self {
        Self {
            ip_provider,
            cp_provider,
        }
    }
    
    /// Get all service alerts from all providers
    pub async fn get_all_alerts(&self) -> Result<Vec<ServiceAlert>, CoreError> {
        let mut all_alerts = Vec::new();
        
        // Get alerts from Infraestruturas de Portugal
        match self.ip_provider.get_service_alerts().await {
            Ok(ip_alerts) => all_alerts.extend(ip_alerts),
            Err(e) => tracing::warn!("Failed to fetch IP alerts: {}", e),
        }
        
        // Get alerts from CP Avisos
        match self.cp_provider.get_service_alerts().await {
            Ok(cp_alerts) => all_alerts.extend(cp_alerts),
            Err(e) => tracing::warn!("Failed to fetch CP avisos: {}", e),
        }
        
        // Deduplicate alerts (same title/content from different sources)
        all_alerts = self.deduplicate_alerts(all_alerts);
        
        Ok(all_alerts)
    }
    
    /// Get alerts affecting a specific station
    pub async fn get_station_alerts(&self, station_id: &str) -> Result<Vec<ServiceAlert>, CoreError> {
        let all_alerts = self.get_all_alerts().await?;
        Ok(all_alerts
            .into_iter()
            .filter(|alert| alert.affects_station(station_id))
            .collect())
    }
    
    /// Get alerts affecting a specific line/service type
    pub async fn get_line_alerts(&self, line: &str) -> Result<Vec<ServiceAlert>, CoreError> {
        let all_alerts = self.get_all_alerts().await?;
        Ok(all_alerts
            .into_iter()
            .filter(|alert| alert.affects_line(line))
            .collect())
    }
    
    /// Deduplicate alerts based on title similarity
    fn deduplicate_alerts(&self, alerts: Vec<ServiceAlert>) -> Vec<ServiceAlert> {
        // Simple deduplication based on title similarity
        // This could be enhanced with more sophisticated matching algorithms
        let mut unique_alerts = Vec::new();
        let mut seen_titles = std::collections::HashSet::new();
        
        for alert in alerts {
            if !seen_titles.contains(&alert.title) {
                seen_titles.insert(alert.title.clone());
                unique_alerts.push(alert);
            }
        }
        
        unique_alerts
    }
}

impl Default for UnifiedAlertSystem {
    fn default() -> Self {
        Self::new()
    }
}