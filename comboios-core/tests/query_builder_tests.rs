use comboios_core::domain::station_timetable::TrainEntry;
use comboios_core::query_builder::{StationQuery, TrainEntryFilter};

fn make_entry(service_type: &str, origin: &str, destination: &str) -> TrainEntry {
    TrainEntry {
        train_number: 100,
        train_number_alt: 0,
        origin_station_name: origin.to_string(),
        destination_station_name: destination.to_string(),
        origin_station_id: 1,
        destination_station_id: 2,
        time: "10:00".to_string(),
        date: "2024-01-01".to_string(),
        observations: String::new(),
        service_type: service_type.to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    }
}

// --- StationQuery ---

#[test]
fn station_query_new_is_empty() {
    let q = StationQuery::new();
    assert_eq!(q.build(), "");
    assert_eq!(q.get_limit(), None);
}

#[test]
fn station_query_build_returns_name() {
    let result = StationQuery::new().name("Lisboa").build();
    assert_eq!(result, "Lisboa");
}

#[test]
fn station_query_build_empty_returns_empty_string() {
    assert_eq!(StationQuery::new().build(), "");
}

#[test]
fn station_query_get_limit_none_when_not_set() {
    assert_eq!(StationQuery::new().get_limit(), None);
}

#[test]
fn station_query_get_limit_some_when_set() {
    assert_eq!(StationQuery::new().limit(5).get_limit(), Some(5));
}

#[test]
fn station_query_name_and_limit_chain() {
    let q = StationQuery::new().name("Porto").limit(3);
    assert_eq!(q.build(), "Porto");
    assert_eq!(q.get_limit(), Some(3));
}

// --- TrainEntryFilter ---

#[test]
fn empty_filter_matches_everything() {
    let filter = TrainEntryFilter::new();
    let entry = make_entry("IC", "Porto Campanha", "Lisboa Oriente");
    assert!(filter.matches(&entry));
}

#[test]
fn service_type_filter_matches_exact() {
    let filter = TrainEntryFilter::new().service_type("IC");
    let entry = make_entry("IC", "Porto Campanha", "Lisboa Oriente");
    assert!(filter.matches(&entry));
}

#[test]
fn service_type_filter_no_match_different_type() {
    let filter = TrainEntryFilter::new().service_type("IC");
    let entry = make_entry("AP", "Porto Campanha", "Lisboa Oriente");
    assert!(!filter.matches(&entry));
}

#[test]
fn service_type_filter_case_insensitive() {
    let filter = TrainEntryFilter::new().service_type("ic");
    let entry = make_entry("IC", "Porto Campanha", "Lisboa Oriente");
    assert!(filter.matches(&entry));
}

#[test]
fn origin_station_filter_substring_match() {
    let filter = TrainEntryFilter::new().origin_station("Porto");
    let entry = make_entry("IC", "Porto Campanha", "Lisboa Oriente");
    assert!(filter.matches(&entry));
}

#[test]
fn origin_station_filter_no_match() {
    let filter = TrainEntryFilter::new().origin_station("Porto");
    let entry = make_entry("IC", "Lisboa Oriente", "Faro");
    assert!(!filter.matches(&entry));
}

#[test]
fn destination_station_filter_substring_match() {
    let filter = TrainEntryFilter::new().destination_station("Lisboa");
    let entry = make_entry("IC", "Porto Campanha", "Lisboa Oriente");
    assert!(filter.matches(&entry));
}

#[test]
fn destination_station_filter_no_match() {
    let filter = TrainEntryFilter::new().destination_station("Lisboa");
    let entry = make_entry("IC", "Porto Campanha", "Faro");
    assert!(!filter.matches(&entry));
}

#[test]
fn multiple_filters_all_must_match() {
    let filter = TrainEntryFilter::new()
        .service_type("IC")
        .origin_station("Porto");

    let matching = make_entry("IC", "Porto Campanha", "Lisboa Oriente");
    assert!(filter.matches(&matching));

    let wrong_service = make_entry("IR", "Porto Campanha", "Lisboa Oriente");
    assert!(!filter.matches(&wrong_service));

    let wrong_origin = make_entry("IC", "Braga", "Lisboa Oriente");
    assert!(!filter.matches(&wrong_origin));
}
