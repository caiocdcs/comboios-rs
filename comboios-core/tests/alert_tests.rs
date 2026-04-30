use chrono::{Duration, Utc};
use comboios_core::domain::alert::{AlertCategory, AlertSeverity, AlertSource, ServiceAlert};

fn make_alert(affected_lines: Vec<&str>, affected_stations: Vec<&str>) -> ServiceAlert {
    ServiceAlert {
        id: "test-1".to_string(),
        title: "Test Alert".to_string(),
        description: "Test description".to_string(),
        severity: AlertSeverity::Info,
        category: AlertCategory::Other,
        affected_lines: affected_lines.into_iter().map(String::from).collect(),
        affected_stations: affected_stations.into_iter().map(String::from).collect(),
        start_time: None,
        end_time: None,
        last_updated: Utc::now(),
        url: None,
        source: AlertSource::ComboiosPortugal,
    }
}

// --- affects_line ---

#[test]
fn affects_line_empty_list_returns_true() {
    let alert = make_alert(vec![], vec![]);
    assert!(alert.affects_line("IC"));
}

#[test]
fn affects_line_matching_entry_returns_true() {
    let alert = make_alert(vec!["IC", "IR"], vec![]);
    assert!(alert.affects_line("IC"));
}

#[test]
fn affects_line_non_matching_entry_returns_false() {
    let alert = make_alert(vec!["IC", "IR"], vec![]);
    assert!(!alert.affects_line("ALFA"));
}

#[test]
fn affects_line_case_insensitive() {
    let alert = make_alert(vec!["IC"], vec![]);
    assert!(alert.affects_line("ic"));
}

// --- affects_station ---

#[test]
fn affects_station_empty_list_returns_true() {
    let alert = make_alert(vec![], vec![]);
    assert!(alert.affects_station("Lisboa Oriente"));
}

#[test]
fn affects_station_matching_entry_returns_true() {
    let alert = make_alert(vec![], vec!["Lisboa Oriente", "Porto Campanha"]);
    assert!(alert.affects_station("Porto Campanha"));
}

#[test]
fn affects_station_non_matching_entry_returns_false() {
    let alert = make_alert(vec![], vec!["Lisboa Oriente"]);
    assert!(!alert.affects_station("Porto Campanha"));
}

#[test]
fn affects_station_case_insensitive() {
    let alert = make_alert(vec![], vec!["Lisboa Oriente"]);
    assert!(alert.affects_station("lisboa oriente"));
}

// --- is_active ---

#[test]
fn is_active_no_times_returns_true() {
    let alert = make_alert(vec![], vec![]);
    assert!(alert.is_active());
}

#[test]
fn is_active_past_start_no_end_returns_true() {
    let mut alert = make_alert(vec![], vec![]);
    alert.start_time = Some(Utc::now() - Duration::hours(1));
    assert!(alert.is_active());
}

#[test]
fn is_active_future_start_returns_false() {
    let mut alert = make_alert(vec![], vec![]);
    alert.start_time = Some(Utc::now() + Duration::hours(1));
    assert!(!alert.is_active());
}

#[test]
fn is_active_past_end_returns_false() {
    let mut alert = make_alert(vec![], vec![]);
    alert.end_time = Some(Utc::now() - Duration::hours(1));
    assert!(!alert.is_active());
}

#[test]
fn is_active_within_start_and_end_returns_true() {
    let mut alert = make_alert(vec![], vec![]);
    alert.start_time = Some(Utc::now() - Duration::hours(1));
    alert.end_time = Some(Utc::now() + Duration::hours(1));
    assert!(alert.is_active());
}

#[test]
fn is_active_before_start_with_end_returns_false() {
    let mut alert = make_alert(vec![], vec![]);
    alert.start_time = Some(Utc::now() + Duration::hours(1));
    alert.end_time = Some(Utc::now() + Duration::hours(2));
    assert!(!alert.is_active());
}
