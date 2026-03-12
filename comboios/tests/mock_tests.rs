use comboios::{ComboiosApi, ApiConfig};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, path_regex};

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
    
    let response_body = r#"{
        "response": [
            {
                "NodeID": 9431039,
                "NomeEstacao": "LISBOA-ORIENTE",
                "TipoPedido": 1,
                "NodesComboioTabelsPartidasChegadas": [
                    {
                        "NComboio1": 722,
                        "NComboio2": 722,
                        "NomeEstacaoOrigem": "BRAGA",
                        "NomeEstacaoDestino": "LISBOA-APOLÓNIA",
                        "EstacaoOrigem": 9429157,
                        "EstacaoDestino": 9430007,
                        "DataHoraPartidaChegada": "17:53",
                        "DataRealizacao": "11-03-2026",
                        "Observacoes": "Circula com atraso de 49 min.",
                        "TipoServico": "IC",
                        "ComboioPassou": false,
                        "Operador": "CP LONGO CURSO"
                    }
                ]
            }
        ]
    }"#;
    
    Mock::given(method("GET"))
        .and(path_regex("/negocios-e-servicos/partidas-chegadas/9431039/.*"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    let api = ComboiosApi::with_config(config);
    
    let result = api.get_station_timetable("9431039", "2026-03-11 06:00", "2026-03-11 20:00").await;
    assert!(result.is_ok());
    
    let boards = result.unwrap();
    assert_eq!(boards.len(), 1);
    assert_eq!(boards[0].station_id, 9431039);
    assert_eq!(boards[0].trains.len(), 1);
    
    let train = &boards[0].trains[0];
    assert_eq!(train.train_number, 722);
    assert_eq!(train.origin_station_name, "BRAGA");
    assert_eq!(train.destination_station_name, "LISBOA-APOLÓNIA");
    assert_eq!(train.time, "17:53");
    assert_eq!(train.delay_minutes(), Some(49));
}

#[tokio::test]
async fn test_get_station_timetable_empty() {
    let mock_server = MockServer::start().await;
    
    let response_body = r#"{"response": []}"#;
    
    Mock::given(method("GET"))
        .and(path_regex("/negocios-e-servicos/partidas-chegadas/9999999/.*"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    let api = ComboiosApi::with_config(config);
    
    let result = api.get_station_timetable("9999999", "2026-03-11 06:00", "2026-03-11 20:00").await;
    assert!(result.is_ok());
    
    let boards = result.unwrap();
    assert!(boards.is_empty());
}

#[tokio::test]
async fn test_delay_parsing() {
    use comboios::TrainEntry;

    // Test various delay patterns
    let entry1 = TrainEntry {
        train_number: 1,
        train_number_alt: 1,
        origin_station_name: "A".to_string(),
        destination_station_name: "B".to_string(),
        origin_station_id: 1,
        destination_station_id: 2,
        time: "10:00".to_string(),
        date: "01-01-2026".to_string(),
        observations: "Circula com atraso de 49 min.".to_string(),
        service_type: "IC".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    };
    assert_eq!(entry1.delay_minutes(), Some(49));

    let entry2 = TrainEntry {
        train_number: 2,
        train_number_alt: 2,
        origin_station_name: "A".to_string(),
        destination_station_name: "B".to_string(),
        origin_station_id: 1,
        destination_station_id: 2,
        time: "10:00".to_string(),
        date: "01-01-2026".to_string(),
        observations: "".to_string(),
        service_type: "IC".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    };
    assert_eq!(entry2.delay_minutes(), None);
}

#[tokio::test]
async fn test_station_id_validation() {
    let mock_server = MockServer::start().await;
    
    let config = ApiConfig {
        cp_base_url: "http://unused".to_string(),
        ip_base_url: mock_server.uri(),
    };
    let api = ComboiosApi::with_config(config);
    
    // Empty station ID should fail
    let result = api.get_station_timetable("", "2026-03-11 06:00", "2026-03-11 20:00").await;
    assert!(result.is_err());
    
    match result.unwrap_err() {
        comboios::CoreError::InvalidInput(msg) => {
            assert!(msg.contains("empty"));
        }
        _ => panic!("Expected InvalidInput error for empty station_id"),
    }
}
