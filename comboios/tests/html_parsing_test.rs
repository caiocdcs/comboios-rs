#[cfg(test)]
mod tests {
    use comboios::{
        domain::alert::AlertSource,
        providers::{alerts::IpAlertsProvider, DataProvider},
    };

    #[test]
    fn test_ip_alerts_provider_creation() {
        let provider = IpAlertsProvider::new();
        assert_eq!(provider.name(), "Infraestruturas de Portugal Alerts");
    }

    #[test]
    fn test_alert_source_display() {
        let source = AlertSource::InfraestruturasPortugal;
        assert_eq!(source.to_string(), "InfraestruturasPortugal");

        let source = AlertSource::ComboiosPortugal;
        assert_eq!(source.to_string(), "ComboiosPortugal");

        let source = AlertSource::UserReported;
        assert_eq!(source.to_string(), "UserReported");
    }
}
