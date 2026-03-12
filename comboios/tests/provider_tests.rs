//! Tests for the new provider-based client

use comboios::{
    ComboiosClient,
    providers::{to_cp_id, to_ip_id},
};

#[tokio::test]
async fn test_ip_provider_client() {
    let client = ComboiosClient::with_ip();
    let result = client.search_stations("Lisboa").await;
    
    // Debug: print the error if it fails
    if let Err(ref e) = result {
        eprintln!("Error: {:?}", e);
    }
    
    // Should succeed
    assert!(result.is_ok(), "Failed to search stations: {:?}", result.err());
    
    let stations = result.unwrap();
    // Should find stations
    assert!(!stations.response.is_empty());
}

#[tokio::test]
async fn test_ip_provider_timetable() {
    let client = ComboiosClient::with_ip();
    
    // Get station board for Lisboa-Oriente
    let result = client.get_station_board_now("9431039").await;
    
    // Debug: print error if failed
    if let Err(ref e) = result {
        eprintln!("Timetable error: {:?}", e);
    }
    
    // Should succeed
    assert!(result.is_ok(), "Failed to get timetable: {:?}", result.err());
    
    let boards = result.unwrap();
    // Should have at least one board (departures)
    assert!(!boards.is_empty());
}

#[test]
fn test_id_mapping() {
    // IP to CP conversion
    assert_eq!(to_cp_id("9431039"), "94-31039");
    assert_eq!(to_cp_id("9402006"), "94-02006");
    
    // CP to IP conversion
    assert_eq!(to_ip_id("94-31039"), "9431039");
    assert_eq!(to_ip_id("94-02006"), "9402006");
    
    // Round-trip
    let original = "9431039";
    let cp = to_cp_id(original);
    let back = to_ip_id(&cp);
    assert_eq!(original, back);
}

#[tokio::test]
async fn test_provider_journey_not_available() {
    // IP provider doesn't support journey
    let client = ComboiosClient::with_ip();
    let result = client.get_train_journey("722").await;
    
    // Should return None (not error)
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}
