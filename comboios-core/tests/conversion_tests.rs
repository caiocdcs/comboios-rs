//! Conversion tests for raw API types to domain types.
//! These are pure unit tests - no network calls.

use comboios_core::domain::cp_types::{
    CpServiceCode, CpStationSimple, CpTrainStop, CpTrainTimetable,
};
use comboios_core::domain::journey::{JourneyStatus, StopStatus};
use comboios_core::domain::station_timetable::TrainEntry;
use comboios_core::domain::train_journey::{IpTrainJourneyResponse, TrainPassage};

// ---------------------------------------------------------------------------
// Fixtures / helpers
// ---------------------------------------------------------------------------

fn make_service_code() -> CpServiceCode {
    CpServiceCode {
        code: "IC".to_string(),
        designation: "Intercidades".to_string(),
    }
}

fn make_station(code: &str, designation: &str) -> CpStationSimple {
    CpStationSimple {
        code: code.to_string(),
        designation: designation.to_string(),
    }
}

fn make_stop(code: &str, designation: &str) -> CpTrainStop {
    CpTrainStop {
        station: make_station(code, designation),
        arrival: Some("10:00".to_string()),
        departure: Some("10:02".to_string()),
        platform: None,
        delay: None,
        eta: None,
        etd: None,
    }
}

fn make_timetable(status: &str, stops: Vec<CpTrainStop>) -> CpTrainTimetable {
    CpTrainTimetable {
        train_number: 720,
        service_code: make_service_code(),
        last_station_code: None,
        delay: None,
        occupancy: None,
        train_stops: stops,
        status: status.to_string(),
        has_disruptions: None,
        duration: None,
        messages: vec![],
    }
}

fn make_ip_passage(
    node_id: u32,
    station_name: &str,
    scheduled_time: &str,
    has_passed: bool,
) -> TrainPassage {
    TrainPassage {
        has_passed,
        scheduled_time: scheduled_time.to_string(),
        node_id,
        station_name: station_name.to_string(),
        observations: String::new(),
    }
}

fn make_ip_response(stops: Vec<TrainPassage>) -> IpTrainJourneyResponse {
    IpTrainJourneyResponse {
        destination_time: "12:00".to_string(),
        origin_time: "10:00".to_string(),
        destination: "Porto Campanha".to_string(),
        duration: "02:00".to_string(),
        stops,
        operator: "CP".to_string(),
        origin: "Lisboa Santa Apolonia".to_string(),
        status: String::new(),
        service_type: "IC".to_string(),
    }
}

fn make_train_entry(observations: &str) -> TrainEntry {
    TrainEntry {
        train_number: 720,
        train_number_alt: 0,
        origin_station_name: "Lisboa".to_string(),
        destination_station_name: "Porto".to_string(),
        origin_station_id: 94001,
        destination_station_id: 94002,
        time: "10:00".to_string(),
        date: "2024-01-01".to_string(),
        observations: observations.to_string(),
        service_type: "IC".to_string(),
        has_passed: false,
        operator: "CP".to_string(),
    }
}

// ---------------------------------------------------------------------------
// CpTrainTimetable::to_train_journey - scheduled train
// ---------------------------------------------------------------------------

#[test]
fn test_cp_scheduled_train_journey_status() {
    let stops = vec![
        make_stop("94-001", "Lisboa Santa Apolonia"),
        make_stop("94-002", "Santarem"),
        make_stop("94-003", "Porto Campanha"),
    ];
    let timetable = make_timetable("SCHEDULED", stops);
    let journey = timetable.to_train_journey();

    assert_eq!(journey.status, JourneyStatus::Scheduled);
}

#[test]
fn test_cp_scheduled_train_all_stops_scheduled() {
    let stops = vec![
        make_stop("94-001", "Lisboa Santa Apolonia"),
        make_stop("94-002", "Santarem"),
        make_stop("94-003", "Porto Campanha"),
    ];
    let timetable = make_timetable("SCHEDULED", stops);
    let journey = timetable.to_train_journey();

    for stop in &journey.stops {
        assert_eq!(
            stop.status,
            StopStatus::Scheduled,
            "stop {} should be Scheduled",
            stop.station.code
        );
    }
}

#[test]
fn test_cp_scheduled_train_number() {
    let timetable = make_timetable("SCHEDULED", vec![make_stop("94-001", "Lisboa")]);
    let journey = timetable.to_train_journey();
    assert_eq!(journey.train_number, "720");
}

#[test]
fn test_cp_scheduled_train_origin_destination() {
    let stops = vec![
        make_stop("94-001", "Lisboa Santa Apolonia"),
        make_stop("94-002", "Santarem"),
        make_stop("94-003", "Porto Campanha"),
    ];
    let timetable = make_timetable("SCHEDULED", stops);
    let journey = timetable.to_train_journey();

    assert_eq!(journey.origin.code, "94-001");
    assert_eq!(journey.origin.designation, "Lisboa Santa Apolonia");
    assert_eq!(journey.destination.code, "94-003");
    assert_eq!(journey.destination.designation, "Porto Campanha");
}

// ---------------------------------------------------------------------------
// CpTrainTimetable::to_train_journey - in-progress train
// ---------------------------------------------------------------------------

#[test]
fn test_cp_in_progress_journey_status() {
    let stops = vec![
        make_stop("94-001", "Lisboa"),
        make_stop("94-002", "Santarem"),
        make_stop("94-003", "Porto"),
    ];
    let mut timetable = make_timetable("NEAR_NEXT", stops);
    // First stop has etd (departed)
    timetable.train_stops[0].etd = Some("10:03".to_string());
    // Second stop has eta but no etd (train is here)
    timetable.train_stops[1].eta = Some("11:00".to_string());
    timetable.last_station_code = Some("94-002".to_string());

    let journey = timetable.to_train_journey();
    assert_eq!(journey.status, JourneyStatus::InProgress);
}

#[test]
fn test_cp_in_progress_first_stop_departed() {
    let stops = vec![
        make_stop("94-001", "Lisboa"),
        make_stop("94-002", "Santarem"),
        make_stop("94-003", "Porto"),
    ];
    let mut timetable = make_timetable("NEAR_NEXT", stops);
    timetable.train_stops[0].etd = Some("10:03".to_string());
    timetable.train_stops[1].eta = Some("11:00".to_string());

    let journey = timetable.to_train_journey();
    // Index 0 with only etd set -> Departed
    assert_eq!(journey.stops[0].status, StopStatus::Departed);
}

#[test]
fn test_cp_in_progress_second_stop_at_stop() {
    let stops = vec![
        make_stop("94-001", "Lisboa"),
        make_stop("94-002", "Santarem"),
        make_stop("94-003", "Porto"),
    ];
    let mut timetable = make_timetable("NEAR_NEXT", stops);
    timetable.train_stops[0].etd = Some("10:03".to_string());
    timetable.train_stops[1].eta = Some("11:00".to_string());

    let journey = timetable.to_train_journey();
    // Second stop has eta but no etd -> AtStop
    assert_eq!(journey.stops[1].status, StopStatus::AtStop);
}

#[test]
fn test_cp_in_progress_third_stop_scheduled() {
    let stops = vec![
        make_stop("94-001", "Lisboa"),
        make_stop("94-002", "Santarem"),
        make_stop("94-003", "Porto"),
    ];
    let mut timetable = make_timetable("NEAR_NEXT", stops);
    timetable.train_stops[0].etd = Some("10:03".to_string());
    timetable.train_stops[1].eta = Some("11:00".to_string());

    let journey = timetable.to_train_journey();
    // Third stop has no eta/etd -> Scheduled
    assert_eq!(journey.stops[2].status, StopStatus::Scheduled);
}

// ---------------------------------------------------------------------------
// CpTrainTimetable::to_train_journey - completed train
// ---------------------------------------------------------------------------

#[test]
fn test_cp_completed_train_passed_status() {
    let timetable = make_timetable(
        "PASSED",
        vec![make_stop("94-001", "Lisboa"), make_stop("94-002", "Porto")],
    );
    let journey = timetable.to_train_journey();
    assert_eq!(journey.status, JourneyStatus::Completed);
}

#[test]
fn test_cp_completed_train_arrived_status() {
    let timetable = make_timetable(
        "ARRIVED",
        vec![make_stop("94-001", "Lisboa"), make_stop("94-002", "Porto")],
    );
    let journey = timetable.to_train_journey();
    assert_eq!(journey.status, JourneyStatus::Completed);
}

// ---------------------------------------------------------------------------
// CpTrainTimetable::to_train_journey - delay propagation
// ---------------------------------------------------------------------------

#[test]
fn test_cp_train_journey_delay_propagated() {
    let mut timetable = make_timetable(
        "SCHEDULED",
        vec![make_stop("94-001", "Lisboa"), make_stop("94-002", "Porto")],
    );
    timetable.delay = Some(15);
    timetable.train_stops[0].delay = Some(15);

    let journey = timetable.to_train_journey();

    assert_eq!(journey.delay_minutes, Some(15));
    assert_eq!(journey.stops[0].delay_minutes, Some(15));
}

#[test]
fn test_cp_stop_delay_independent_from_journey_delay() {
    let mut timetable = make_timetable(
        "SCHEDULED",
        vec![make_stop("94-001", "Lisboa"), make_stop("94-002", "Porto")],
    );
    timetable.delay = Some(10);
    timetable.train_stops[0].delay = Some(5);
    timetable.train_stops[1].delay = Some(10);

    let journey = timetable.to_train_journey();

    assert_eq!(journey.delay_minutes, Some(10));
    assert_eq!(journey.stops[0].delay_minutes, Some(5));
    assert_eq!(journey.stops[1].delay_minutes, Some(10));
}

#[test]
fn test_cp_train_no_delay_is_none() {
    let timetable = make_timetable("SCHEDULED", vec![make_stop("94-001", "Lisboa")]);
    let journey = timetable.to_train_journey();
    assert_eq!(journey.delay_minutes, None);
    assert_eq!(journey.stops[0].delay_minutes, None);
}

// ---------------------------------------------------------------------------
// CpTrainTimetable::to_train_journey - stop numbering and fields
// ---------------------------------------------------------------------------

#[test]
fn test_cp_stop_numbers_are_one_based() {
    let stops = vec![
        make_stop("94-001", "Lisboa"),
        make_stop("94-002", "Santarem"),
        make_stop("94-003", "Porto"),
    ];
    let timetable = make_timetable("SCHEDULED", stops);
    let journey = timetable.to_train_journey();

    for (i, stop) in journey.stops.iter().enumerate() {
        assert_eq!(stop.stop_number, i + 1);
    }
}

#[test]
fn test_cp_service_type_formatted() {
    let timetable = make_timetable("SCHEDULED", vec![make_stop("94-001", "Lisboa")]);
    let journey = timetable.to_train_journey();
    assert_eq!(journey.service_type, "IC|Intercidades");
}

#[test]
fn test_cp_operator_is_cp() {
    let timetable = make_timetable("SCHEDULED", vec![make_stop("94-001", "Lisboa")]);
    let journey = timetable.to_train_journey();
    assert_eq!(journey.operator, "CP");
}

// ---------------------------------------------------------------------------
// IpTrainJourneyResponse::to_train_journey
// ---------------------------------------------------------------------------

#[test]
fn test_ip_train_number_matches() {
    let response = make_ip_response(vec![make_ip_passage(94001, "Lisboa", "10:00", false)]);
    let journey = response.to_train_journey("720");
    assert_eq!(journey.train_number, "720");
}

#[test]
fn test_ip_origin_designation_matches_response() {
    let response = make_ip_response(vec![make_ip_passage(94001, "Lisboa", "10:00", false)]);
    let journey = response.to_train_journey("720");
    assert_eq!(journey.origin.designation, "Lisboa Santa Apolonia");
}

#[test]
fn test_ip_destination_designation_matches_response() {
    let response = make_ip_response(vec![make_ip_passage(94001, "Lisboa", "10:00", false)]);
    let journey = response.to_train_journey("720");
    assert_eq!(journey.destination.designation, "Porto Campanha");
}

#[test]
fn test_ip_passed_stop_has_passed_status() {
    let response = make_ip_response(vec![
        make_ip_passage(94001, "Lisboa", "10:00", true),
        make_ip_passage(94002, "Santarem", "11:00", false),
        make_ip_passage(94003, "Porto", "12:00", false),
    ]);
    let journey = response.to_train_journey("720");
    assert_eq!(journey.stops[0].status, StopStatus::Passed);
}

#[test]
fn test_ip_stop_count_matches_passages() {
    let response = make_ip_response(vec![
        make_ip_passage(94001, "Lisboa", "10:00", true),
        make_ip_passage(94002, "Santarem", "11:00", false),
        make_ip_passage(94003, "Porto", "12:00", false),
    ]);
    let journey = response.to_train_journey("720");
    assert_eq!(journey.stops.len(), 3);
}

#[test]
fn test_ip_station_name_and_node_id_mapped() {
    let response = make_ip_response(vec![make_ip_passage(
        94001,
        "Lisboa Santa Apolonia",
        "10:00",
        false,
    )]);
    let journey = response.to_train_journey("720");
    assert_eq!(
        journey.stops[0].station.designation,
        "Lisboa Santa Apolonia"
    );
    assert_eq!(journey.stops[0].station.code, "94001");
}

#[test]
fn test_ip_stop_numbers_are_one_based() {
    let response = make_ip_response(vec![
        make_ip_passage(94001, "Lisboa", "10:00", false),
        make_ip_passage(94002, "Porto", "12:00", false),
    ]);
    let journey = response.to_train_journey("720");
    assert_eq!(journey.stops[0].stop_number, 1);
    assert_eq!(journey.stops[1].stop_number, 2);
}

#[test]
fn test_ip_service_type_propagated() {
    let response = make_ip_response(vec![make_ip_passage(94001, "Lisboa", "10:00", false)]);
    let journey = response.to_train_journey("720");
    assert_eq!(journey.service_type, "IC");
}

#[test]
fn test_ip_operator_propagated() {
    let response = make_ip_response(vec![make_ip_passage(94001, "Lisboa", "10:00", false)]);
    let journey = response.to_train_journey("720");
    assert_eq!(journey.operator, "CP");
}

// ---------------------------------------------------------------------------
// TrainEntry::delay_minutes - parsing from observations string
// ---------------------------------------------------------------------------

#[test]
fn test_delay_minutes_15() {
    let entry = make_train_entry("Comboio com atraso de 15 min.");
    assert_eq!(entry.delay_minutes(), Some(15));
}

#[test]
fn test_delay_minutes_6() {
    let entry = make_train_entry("Comboio com atraso de 6 min.");
    assert_eq!(entry.delay_minutes(), Some(6));
}

#[test]
fn test_delay_minutes_empty_observations() {
    let entry = make_train_entry("");
    assert_eq!(entry.delay_minutes(), None);
}

#[test]
fn test_delay_minutes_no_delay_text() {
    let entry = make_train_entry("A tempo");
    assert_eq!(entry.delay_minutes(), None);
}

#[test]
fn test_delay_minutes_case_insensitive() {
    let entry = make_train_entry("ATRASO DE 5 MIN");
    assert_eq!(entry.delay_minutes(), Some(5));
}

#[test]
fn test_delay_minutes_zero_not_returned_when_absent() {
    let entry = make_train_entry("Sem perturbacoes");
    assert_eq!(entry.delay_minutes(), None);
}
