use comboios::{ComboiosApi, ApiConfig, RetryConfig, CoreError};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};
use std::time::Duration;

#[tokio::test]
async fn test_retry_on_server_error_success() {
    let mock_server = MockServer::start().await;
    
    let response_body = r#"{
        "response": [{"NodeID": 123, "Nome": "Test Station"}]
    }"#;
    
    // First request fails, second succeeds
    Mock::given(method("GET"))
        .and(path("/negocios-e-servicos/estacao-nome/Test"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    
    Mock::given(method("GET"))
        .and(path("/negocios-e-servicos/estacao-nome/Test"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    
    let retry_config = RetryConfig {
        max_retries: 3,
        base_delay: Duration::from_millis(10),
        max_delay: Duration::from_millis(100),
        retry_on_server_errors: true,
        retry_on_network_errors: true,
    };
    
    let api = ComboiosApi::with_config(config)
        .with_retry(retry_config);
    
    let result = api.get_stations("Test").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().response.len(), 1);
}

#[tokio::test]
async fn test_retry_exhausted() {
    let mock_server = MockServer::start().await;
    
    // All requests fail
    Mock::given(method("GET"))
        .and(path("/negocios-e-servicos/estacao-nome/Fail"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Server Error"))
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    
    let retry_config = RetryConfig {
        max_retries: 2,
        base_delay: Duration::from_millis(1),
        max_delay: Duration::from_millis(10),
        retry_on_server_errors: true,
        retry_on_network_errors: true,
    };
    
    let api = ComboiosApi::with_config(config)
        .with_retry(retry_config);
    
    let result = api.get_stations("Fail").await;
    assert!(result.is_err());
    
    match result.unwrap_err() {
        CoreError::ApiError { status, message } => {
            assert_eq!(status, 500);
            assert_eq!(message, "Server Error");
        }
        _ => panic!("Expected ApiError"),
    }
}

#[tokio::test]
async fn test_no_retry_on_client_error() {
    let mock_server = MockServer::start().await;
    
    // 404 should not be retried
    Mock::given(method("GET"))
        .and(path("/negocios-e-servicos/estacao-nome/NotFound"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1) // Should only be called once
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    
    let retry_config = RetryConfig {
        max_retries: 3,
        base_delay: Duration::from_millis(10),
        max_delay: Duration::from_millis(100),
        retry_on_server_errors: true,
        retry_on_network_errors: true,
    };
    
    let api = ComboiosApi::with_config(config)
        .with_retry(retry_config);
    
    let result = api.get_stations("NotFound").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_retry_disabled() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/negocios-e-servicos/estacao-nome/NoRetry"))
        .respond_with(ResponseTemplate::new(500))
        .expect(1) // Should only be called once when retry is disabled
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    
    let api = ComboiosApi::with_config(config)
        .with_retry(RetryConfig::disabled());
    
    let result = api.get_stations("NoRetry").await;
    assert!(result.is_err());
}
