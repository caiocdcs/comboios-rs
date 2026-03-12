use comboios::{StationQuery, TrainEntryFilter};

#[test]
fn test_station_query_builder() {
    let query = StationQuery::new().name("Lisboa").limit(10);

    assert_eq!(query.build(), "Lisboa");
    assert_eq!(query.get_limit(), Some(10));
}

#[test]
fn test_station_query_default() {
    let query = StationQuery::new();
    assert_eq!(query.build(), "");
    assert_eq!(query.get_limit(), None);
}

#[test]
fn test_station_query_chaining() {
    let query = StationQuery::new().name("Porto").limit(5);

    assert_eq!(query.build(), "Porto");
    assert_eq!(query.get_limit(), Some(5));
}

#[test]
fn test_train_entry_filter_empty() {
    use comboios::TrainEntry;

    let filter = TrainEntryFilter::new();

    let entry = TrainEntry {
        train_number: 12345,
        train_number_alt: 12345,
        origin_station_name: "Lisboa".to_string(),
        destination_station_name: "Porto".to_string(),
        origin_station_id: 9430007,
        destination_station_id: 9402006,
        time: "14:30".to_string(),
        date: "11-03-2026".to_string(),
        observations: "".to_string(),
        service_type: "IC".to_string(),
        has_passed: false,
        operator: "CP LONGO CURSO".to_string(),
    };

    assert!(filter.matches(&entry));
}

#[test]
fn test_train_entry_filter_by_service_type() {
    use comboios::TrainEntry;

    let filter = TrainEntryFilter::new().service_type("IC");

    let ic_train = TrainEntry {
        train_number: 123,
        train_number_alt: 123,
        origin_station_name: "Lisboa".to_string(),
        destination_station_name: "Porto".to_string(),
        origin_station_id: 1,
        destination_station_id: 2,
        time: "14:30".to_string(),
        date: "11-03-2026".to_string(),
        observations: "".to_string(),
        service_type: "IC".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    };

    let alfa_train = TrainEntry {
        train_number: 124,
        train_number_alt: 124,
        origin_station_name: "Lisboa".to_string(),
        destination_station_name: "Porto".to_string(),
        origin_station_id: 1,
        destination_station_id: 2,
        time: "15:00".to_string(),
        date: "11-03-2026".to_string(),
        observations: "".to_string(),
        service_type: "ALFA".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    };

    assert!(filter.matches(&ic_train));
    assert!(!filter.matches(&alfa_train));
}

#[test]
fn test_train_entry_filter_by_origin() {
    use comboios::TrainEntry;

    let filter = TrainEntryFilter::new().origin_station("Lisboa");

    let from_lisboa = TrainEntry {
        train_number: 123,
        train_number_alt: 123,
        origin_station_name: "LISBOA-ORIENTE".to_string(),
        destination_station_name: "Porto".to_string(),
        origin_station_id: 9431039,
        destination_station_id: 9402006,
        time: "14:30".to_string(),
        date: "11-03-2026".to_string(),
        observations: "".to_string(),
        service_type: "IC".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    };

    let from_braga = TrainEntry {
        train_number: 124,
        train_number_alt: 124,
        origin_station_name: "BRAGA".to_string(),
        destination_station_name: "Lisboa".to_string(),
        origin_station_id: 9429157,
        destination_station_id: 9431039,
        time: "15:00".to_string(),
        date: "11-03-2026".to_string(),
        observations: "".to_string(),
        service_type: "IC".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    };

    assert!(filter.matches(&from_lisboa));
    assert!(!filter.matches(&from_braga));
}

#[test]
fn test_train_entry_filter_by_destination() {
    use comboios::TrainEntry;

    let filter = TrainEntryFilter::new().destination_station("Porto");

    let to_porto = TrainEntry {
        train_number: 123,
        train_number_alt: 123,
        origin_station_name: "Lisboa".to_string(),
        destination_station_name: "PORTO-CAMPANHÃ".to_string(),
        origin_station_id: 9430007,
        destination_station_id: 9402006,
        time: "14:30".to_string(),
        date: "11-03-2026".to_string(),
        observations: "".to_string(),
        service_type: "IC".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    };

    let to_braga = TrainEntry {
        train_number: 124,
        train_number_alt: 124,
        origin_station_name: "Lisboa".to_string(),
        destination_station_name: "BRAGA".to_string(),
        origin_station_id: 9430007,
        destination_station_id: 9429157,
        time: "15:00".to_string(),
        date: "11-03-2026".to_string(),
        observations: "".to_string(),
        service_type: "IC".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    };

    assert!(filter.matches(&to_porto));
    assert!(!filter.matches(&to_braga));
}

#[test]
fn test_train_entry_filter_combined() {
    use comboios::TrainEntry;

    let filter = TrainEntryFilter::new()
        .service_type("IC")
        .origin_station("Lisboa")
        .destination_station("Porto");

    let matching = TrainEntry {
        train_number: 123,
        train_number_alt: 123,
        origin_station_name: "LISBOA-ORIENTE".to_string(),
        destination_station_name: "PORTO-CAMPANHÃ".to_string(),
        origin_station_id: 9431039,
        destination_station_id: 9402006,
        time: "14:30".to_string(),
        date: "11-03-2026".to_string(),
        observations: "".to_string(),
        service_type: "IC".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    };

    let wrong_service = TrainEntry {
        train_number: 124,
        train_number_alt: 124,
        origin_station_name: "LISBOA-ORIENTE".to_string(),
        destination_station_name: "PORTO-CAMPANHÃ".to_string(),
        origin_station_id: 9431039,
        destination_station_id: 9402006,
        time: "15:00".to_string(),
        date: "11-03-2026".to_string(),
        observations: "".to_string(),
        service_type: "ALFA".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    };

    assert!(filter.matches(&matching));
    assert!(!filter.matches(&wrong_service));
}
