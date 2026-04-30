//! Tests for station and timetable domain models

use comboios_core::domain::station::Station;
use comboios_core::domain::station_timetable::{StationBoard, StationTimetable};

#[test]
fn test_station_creation() {
    let station = Station {
        code: "94-31039".to_string(),
        designation: "Lisboa Oriente".to_string(),
    };

    assert_eq!(station.code, "94-31039");
    assert_eq!(station.designation, "Lisboa Oriente");
}

#[test]
fn test_station_serialization() {
    let station = Station {
        code: "94-2006".to_string(),
        designation: "Porto Campanha".to_string(),
    };

    let json = serde_json::to_string(&station).unwrap();
    assert!(json.contains("94-2006"));
    assert!(json.contains("Porto Campanha"));

    let deserialized: Station = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.code, station.code);
    assert_eq!(deserialized.designation, station.designation);
}

#[test]
fn test_station_timetable_creation() {
    let timetable = StationTimetable {
        train_number: 120,
        service_type: "AP|Alfa Pendular".to_string(),
        origin_station_name: "Porto Campanha".to_string(),
        origin_station_id: "94-2006".to_string(),
        destination_station_name: "Lisboa Santa Apolonia".to_string(),
        destination_station_id: "94-30007".to_string(),
        departure_time: Some("12:23".to_string()),
        arrival_time: Some("12:22".to_string()),
        platform: Some("8".to_string()),
        delay: Some(5),
        operator: "CP".to_string(),
        has_passed: false,
        is_departure: true,
    };

    assert_eq!(timetable.train_number, 120);
    assert!(timetable.is_departure);
    assert!(!timetable.has_passed);
}

#[test]
fn test_station_board_creation() {
    let board = StationBoard {
        station_id: "94-31039".to_string(),
        station_name: "Lisboa Oriente".to_string(),
        trains: vec![StationTimetable {
            train_number: 120,
            service_type: "AP|Alfa Pendular".to_string(),
            origin_station_name: "Porto".to_string(),
            origin_station_id: "94-2006".to_string(),
            destination_station_name: "Lisboa".to_string(),
            destination_station_id: "94-30007".to_string(),
            departure_time: Some("12:23".to_string()),
            arrival_time: Some("12:22".to_string()),
            platform: Some("8".to_string()),
            delay: None,
            operator: "CP".to_string(),
            has_passed: false,
            is_departure: true,
        }],
    };

    assert_eq!(board.station_id, "94-31039");
    assert_eq!(board.trains.len(), 1);
}

#[test]
fn test_station_timetable_serialization() {
    let timetable = StationTimetable {
        train_number: 500,
        service_type: "IC|Intercidades".to_string(),
        origin_station_name: "Faro".to_string(),
        origin_station_id: "94-70006".to_string(),
        destination_station_name: "Lisboa".to_string(),
        destination_station_id: "94-30007".to_string(),
        departure_time: None,
        arrival_time: Some("15:30".to_string()),
        platform: None,
        delay: Some(10),
        operator: "CP".to_string(),
        has_passed: true,
        is_departure: false,
    };

    let json = serde_json::to_string(&timetable).unwrap();
    let deserialized: StationTimetable = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.train_number, 500);
    assert_eq!(deserialized.delay, Some(10));
    assert!(deserialized.has_passed);
    assert!(!deserialized.is_departure);
}

#[test]
fn test_station_board_serialization() {
    let board = StationBoard {
        station_id: "94-2006".to_string(),
        station_name: "Porto Campanha".to_string(),
        trains: vec![],
    };

    let json = serde_json::to_string(&board).unwrap();
    let deserialized: StationBoard = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.station_name, "Porto Campanha");
    assert!(deserialized.trains.is_empty());
}
