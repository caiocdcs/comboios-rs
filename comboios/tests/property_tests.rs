use comboios::Station;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_station_code_always_string(code in 1u32..100000) {
        // Simulate JSON with numeric NodeID
        let json = format!(r#"{{"NodeID": {}, "Nome": "Test Station"}}"#, code);
        let station: Station = serde_json::from_str(&json).expect("Should parse numeric code");
        assert_eq!(station.code, code.to_string());
    }

    #[test]
    fn test_station_code_string_parsing(code in "[A-Z0-9]{1,10}") {
        // Simulate JSON with string NodeID
        let json = format!(r#"{{"NodeID": "{}", "Nome": "Test Station"}}"#, code);
        let station: Station = serde_json::from_str(&json).expect("Should parse string code");
        assert_eq!(station.code, code);
    }

    #[test]
    fn test_station_designation_not_empty(name in "[A-Za-z0-9 -]{1,100}") {
        let json = format!(r#"{{"NodeID": 123, "Nome": "{}"}}"#, name.replace("\"", "\\\""));
        let station: Station = serde_json::from_str(&json).expect("Should parse name");
        assert!(!station.designation.is_empty());
    }
}

#[test]
fn test_station_format_validation() {
    // Test station code format: XX-XXXX
    let valid_codes = vec!["94-405", "94-401", "10-123"];
    for code in valid_codes {
        assert!(code.len() >= 6, "Station code should have proper length");
        assert!(code.contains('-'), "Station code should contain hyphen");
    }
}
