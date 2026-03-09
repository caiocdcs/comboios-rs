use comboios::{ComboiosApi, ApiConfig};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_get_stations_success() {
    let mock_server = MockServer::start().await;
    
    let response_body = r#"{
        "response": [
            {
                "NodeID": 123,
                "Nome": "Lisboa - Santa Apolonia"
            },
            {
                "NodeID": 456,
                "Nome": "Lisboa - Oriente"
            }
        ]
    }"#;
    
    Mock::given(method("GET"))
        .and(path("/negocios-e-servicos/estacao-nome/Lisboa"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    let api = ComboiosApi::with_config(config);
    
    let result = api.get_stations("Lisboa").await;
    assert!(result.is_ok());
    
    let stations = result.unwrap();
    assert_eq!(stations.response.len(), 2);
    assert_eq!(stations.response[0].designation, "Lisboa - Santa Apolonia");
    assert_eq!(stations.response[0].code, "123");
}

#[tokio::test]
async fn test_get_stations_not_found() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/negocios-e-servicos/estacao-nome/NonExistent"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not found"))
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    let api = ComboiosApi::with_config(config);
    
    let result = api.get_stations("NonExistent").await;
    assert!(result.is_err());
    
    match result.unwrap_err() {
        comboios::CoreError::ApiError { status, .. } => {
            assert_eq!(status, 404);
        }
        _ => panic!("Expected ApiError"),
    }
}

#[tokio::test]
async fn test_get_stations_malformed_json() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/negocios-e-servicos/estacao-nome/Bad"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not valid json"))
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    let api = ComboiosApi::with_config(config);
    
    let result = api.get_stations("Bad").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_stations_empty_response() {
    let mock_server = MockServer::start().await;
    
    let response_body = r#"{"response": []}"#;
    
    Mock::given(method("GET"))
        .and(path("/negocios-e-servicos/estacao-nome/XYZ"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    let api = ComboiosApi::with_config(config);
    
    let result = api.get_stations("XYZ").await;
    assert!(result.is_ok());
    
    let stations = result.unwrap();
    assert!(stations.response.is_empty());
}

#[tokio::test]
async fn test_get_station_timetable_success() {
    let mock_server = MockServer::start().await;
    
    let response_body = r#"[
        {
            "delay": 5,
            "trainOrigin": {
                "NodeID": 123,
                "Nome": "Lisboa - Santa Apolonia"
            },
            "trainDestination": {
                "NodeID": 456,
                "Nome": "Porto - Campanha"
            },
            "departureTime": "14:30",
            "arrivalTime": "17:45",
            "trainNumber": 12345,
            "platform": "3"
        }
    ]"#;
    
    Mock::given(method("GET"))
        .and(path("/station/trains"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: mock_server.uri(),
        ip_base_url: "http://unused".to_string(),
    };
    let api = ComboiosApi::with_config(config);
    
    let result = api.get_station_timetable("94405").await;
    assert!(result.is_ok());
    
    let timetable = result.unwrap();
    assert_eq!(timetable.len(), 1);
    assert_eq!(timetable[0].train_number, 12345);
    assert_eq!(timetable[0].delay, Some(5));
}
