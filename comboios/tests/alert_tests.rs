#[cfg(test)]
mod tests {
    use comboios::{
        alerts::UnifiedAlertSystem,
        domain::alert::{ServiceAlert, AlertSource},
    };

    #[tokio::test]
    async fn test_unified_alert_system() {
        let unified_system = UnifiedAlertSystem::new();
        let alerts = unified_system.get_all_alerts().await.unwrap();
        
        // Should have alerts from both IP and CP sources
        assert!(!alerts.is_empty());
        assert!(alerts.len() >= 2); // At least one from each source
        
        // Check that we have alerts from both sources
        let ip_alerts: Vec<&ServiceAlert> = alerts.iter()
            .filter(|a| matches!(a.source, AlertSource::InfraestruturasPortugal))
            .collect();
        
        let cp_alerts: Vec<&ServiceAlert> = alerts.iter()
            .filter(|a| matches!(a.source, AlertSource::ComboiosPortugal))
            .collect();
        
        assert!(!ip_alerts.is_empty());
        assert!(!cp_alerts.is_empty());
    }

    #[tokio::test]
    async fn test_alert_filtering() {
        let unified_system = UnifiedAlertSystem::new();
        
        // Test station filtering
        let station_alerts = unified_system.get_station_alerts("Lisboa").await.unwrap();
        assert!(!station_alerts.is_empty());
        
        // Test line filtering
        let line_alerts = unified_system.get_line_alerts("IC").await.unwrap();
        assert!(!line_alerts.is_empty());
    }
}