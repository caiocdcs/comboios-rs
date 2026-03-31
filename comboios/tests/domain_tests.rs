//! Tests for domain models

use comboios::domain::journey::{JourneyStatus, JourneyStop, StopStatus, TrainJourney};
use comboios::domain::station::Station;

fn make_stop(num: usize, status: StopStatus) -> JourneyStop {
    JourneyStop {
        station: Station {
            code: format!("ST{}", num),
            designation: format!("Station {}", num),
        },
        scheduled_arrival: format!("{:02}:00", 8 + num),
        actual_arrival: if status == StopStatus::Departed || status == StopStatus::AtStop {
            Some(format!("{:02}:05", 8 + num))
        } else {
            None
        },
        scheduled_departure: format!("{:02}:05", 8 + num),
        actual_departure: if status == StopStatus::Departed {
            Some(format!("{:02}:10", 8 + num))
        } else {
            None
        },
        platform: Some("1".to_string()),
        status: status.clone(),
        delay_minutes: Some(5),
        stop_number: num,
        has_passed: Some(status == StopStatus::Departed || status == StopStatus::Passed),
        predicted_time: None,
    }
}

fn make_journey(stops: Vec<JourneyStop>) -> TrainJourney {
    let origin = stops.first().map(|s| s.station.clone()).unwrap_or(Station {
        code: "".to_string(),
        designation: "".to_string(),
    });
    let destination = stops.last().map(|s| s.station.clone()).unwrap_or(Station {
        code: "".to_string(),
        designation: "".to_string(),
    });

    TrainJourney {
        train_number: "123".to_string(),
        service_type: "IC".to_string(),
        origin,
        destination,
        stops,
        status: JourneyStatus::InProgress,
        delay_minutes: Some(5),
        operator: "CP".to_string(),
        observations: None,
        duration: Some("02:30".to_string()),
    }
}

#[test]
fn test_journey_stop_has_arrived() {
    let departed = make_stop(1, StopStatus::Departed);
    let at_stop = make_stop(2, StopStatus::AtStop);
    let scheduled = make_stop(3, StopStatus::Scheduled);

    assert!(departed.has_arrived());
    assert!(at_stop.has_arrived());
    assert!(!scheduled.has_arrived());
}

#[test]
fn test_journey_stop_has_departed() {
    let departed = make_stop(1, StopStatus::Departed);
    let at_stop = make_stop(2, StopStatus::AtStop);
    let scheduled = make_stop(3, StopStatus::Scheduled);

    assert!(departed.has_departed());
    assert!(!at_stop.has_departed());
    assert!(!scheduled.has_departed());
}

#[test]
fn test_journey_stop_is_future() {
    let departed = make_stop(1, StopStatus::Departed);
    let scheduled = make_stop(2, StopStatus::Scheduled);

    assert!(!departed.is_future());
    assert!(scheduled.is_future());
}

#[test]
fn test_journey_stop_display_times() {
    let stop = make_stop(1, StopStatus::Departed);

    assert_eq!(stop.display_arrival(), "09:05");
    assert_eq!(stop.display_departure(), "09:10");

    let scheduled_stop = make_stop(2, StopStatus::Scheduled);
    assert_eq!(scheduled_stop.display_arrival(), "10:00");
    assert_eq!(scheduled_stop.display_departure(), "10:05");
}

#[test]
fn test_train_journey_current_stop() {
    let stops = vec![
        make_stop(1, StopStatus::Departed),
        make_stop(2, StopStatus::AtStop),
        make_stop(3, StopStatus::Scheduled),
    ];
    let journey = make_journey(stops);

    let current = journey.current_stop().unwrap();
    assert_eq!(current.station.code, "ST2");
}

#[test]
fn test_train_journey_current_stop_when_no_at_stop() {
    let stops = vec![
        make_stop(1, StopStatus::Departed),
        make_stop(2, StopStatus::Departed),
        make_stop(3, StopStatus::Scheduled),
    ];
    let journey = make_journey(stops);

    let current = journey.current_stop().unwrap();
    assert_eq!(current.station.code, "ST3");
}

#[test]
fn test_train_journey_upcoming_stops() {
    let stops = vec![
        make_stop(1, StopStatus::Departed),
        make_stop(2, StopStatus::AtStop),
        make_stop(3, StopStatus::Scheduled),
        make_stop(4, StopStatus::Scheduled),
    ];
    let journey = make_journey(stops);

    let upcoming = journey.upcoming_stops(3);
    assert_eq!(upcoming.len(), 3);
    assert_eq!(upcoming[0].station.code, "ST2"); // AtStop included
    assert_eq!(upcoming[1].station.code, "ST3");
    assert_eq!(upcoming[2].station.code, "ST4");
}

#[test]
fn test_train_journey_completed_stops() {
    let stops = vec![
        make_stop(1, StopStatus::Departed),
        make_stop(2, StopStatus::Departed),
        make_stop(3, StopStatus::AtStop),
        make_stop(4, StopStatus::Scheduled),
    ];
    let journey = make_journey(stops);

    let completed = journey.completed_stops();
    assert_eq!(completed.len(), 2);
}

#[test]
fn test_train_journey_progress_percent() {
    let stops = vec![
        make_stop(1, StopStatus::Departed),
        make_stop(2, StopStatus::Departed),
        make_stop(3, StopStatus::AtStop),
        make_stop(4, StopStatus::Scheduled),
    ];
    let journey = make_journey(stops);

    assert_eq!(journey.progress_percent(), 50); // 2 of 4 departed
}

#[test]
fn test_train_journey_progress_percent_empty() {
    let journey = make_journey(vec![]);
    assert_eq!(journey.progress_percent(), 0);
}

#[test]
fn test_journey_status_serialization() {
    let status = JourneyStatus::InProgress;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, "\"IN_PROGRESS\"");

    let deserialized: JourneyStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, JourneyStatus::InProgress);
}

#[test]
fn test_stop_status_serialization() {
    let status = StopStatus::AtStop;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, "\"AT_STOP\"");
}
