use comboios::{Station, StationQuery, Timetable, TimetableFilter};

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
fn test_timetable_filter_empty() {
    let filter = TimetableFilter::new();

    let timetable = Timetable {
        delay: None,
        train_origin: Station {
            code: "123".to_string(),
            designation: "Origin".to_string(),
        },
        train_destination: Station {
            code: "456".to_string(),
            designation: "Dest".to_string(),
        },
        departure_time: Some("14:30".to_string()),
        arrival_time: Some("17:45".to_string()),
        train_number: 12345,
        platform: Some("3".to_string()),
        occupancy: None,
        eta: None,
        etd: None,
    };

    assert!(filter.matches(&timetable));
}

#[test]
fn test_timetable_filter_by_platform() {
    let filter = TimetableFilter::new().platform("3");

    let timetable_match = Timetable {
        delay: None,
        train_origin: Station {
            code: "123".to_string(),
            designation: "Origin".to_string(),
        },
        train_destination: Station {
            code: "456".to_string(),
            designation: "Dest".to_string(),
        },
        departure_time: Some("14:30".to_string()),
        arrival_time: Some("17:45".to_string()),
        train_number: 12345,
        platform: Some("3".to_string()),
        occupancy: None,
        eta: None,
        etd: None,
    };

    let timetable_no_match = Timetable {
        delay: None,
        train_origin: Station {
            code: "123".to_string(),
            designation: "Origin".to_string(),
        },
        train_destination: Station {
            code: "456".to_string(),
            designation: "Dest".to_string(),
        },
        departure_time: Some("14:30".to_string()),
        arrival_time: Some("17:45".to_string()),
        train_number: 12346,
        platform: Some("4".to_string()),
        occupancy: None,
        eta: None,
        etd: None,
    };

    assert!(filter.matches(&timetable_match));
    assert!(!filter.matches(&timetable_no_match));
}
