#[cfg(test)]
mod tests {
    use comboios::providers::alerts::IpAlertsProvider;
    use comboios::providers::DataProvider;

    #[tokio::test]
    async fn test_ip_alerts_provider_creation() {
        let provider = IpAlertsProvider::new();
        assert_eq!(provider.name(), "Infraestruturas de Portugal Alerts");
    }

    #[tokio::test]
    async fn test_ip_alerts_url() {
        let _provider = IpAlertsProvider::new();
    }
}