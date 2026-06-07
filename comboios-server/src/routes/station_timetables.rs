use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use chrono::{Local, TimeZone};
use comboios_core::domain::station_timetable::StationBoard;

use crate::{
    domain::{AppResponse, AppState},
    error::AppError,
};

/// # Errors
///
/// Returns [`AppError`] if the CP API call fails.
#[tracing::instrument(skip(state))]
pub async fn station_timetables(
    State(state): State<Arc<AppState>>,
    Path(station_id): Path<String>,
) -> Result<Json<AppResponse<Vec<StationBoard>>>, AppError> {
    tracing::info!("Finding timetable for station {}", station_id);

    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();
    let start_time = now.format("%H:%M").to_string();

    let mut boards = state
        .api
        .get_station_timetable(&station_id, &date, Some(&start_time))
        .await?;

    let today = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok();
    for board in &mut boards.response {
        if board.station_name.is_empty()
            && let Ok(names) = state.station_names.read()
            && let Some(name) = names.get(&board.station_id)
        {
            board.station_name = name.clone();
        }

        for train in &mut board.trains {
            train.has_passed = compute_has_passed(
                &train.estimated_departure,
                &train.departure_time,
                &train.estimated_arrival,
                &train.arrival_time,
                today,
                now,
            );
        }
    }

    Ok(Json(AppResponse {
        data: boards.response,
    }))
}

/// Determine whether a train has already passed the station by comparing its
/// best available time (estimated departure, scheduled departure, estimated
/// arrival, scheduled arrival) against the current wall-clock time.
fn compute_has_passed(
    estimated_departure: &Option<String>,
    departure_time: &Option<String>,
    estimated_arrival: &Option<String>,
    arrival_time: &Option<String>,
    today: Option<chrono::NaiveDate>,
    now: chrono::DateTime<chrono::Local>,
) -> bool {
    let time_to_check = estimated_departure
        .as_ref()
        .or(departure_time.as_ref())
        .or(estimated_arrival.as_ref())
        .or(arrival_time.as_ref());

    if let Some(time) = time_to_check
        && let (Some(today_date), Ok(t)) = (today, chrono::NaiveTime::parse_from_str(time, "%H:%M"))
    {
        let train_naive = today_date.and_time(t);
        return chrono::Local
            .from_local_datetime(&train_naive)
            .single()
            .map(|dt| dt <= now)
            .unwrap_or(false);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, TimeZone};

    #[test]
    fn train_in_the_past_is_passed() {
        let now = Local.with_ymd_and_hms(2024, 6, 7, 14, 0, 0).unwrap();
        let today = chrono::NaiveDate::from_ymd_opt(2024, 6, 7);
        assert!(compute_has_passed(
            &None,
            &Some("13:30".to_string()),
            &None,
            &None,
            today,
            now,
        ));
    }

    #[test]
    fn train_in_the_future_is_not_passed() {
        let now = Local.with_ymd_and_hms(2024, 6, 7, 14, 0, 0).unwrap();
        let today = chrono::NaiveDate::from_ymd_opt(2024, 6, 7);
        assert!(!compute_has_passed(
            &None,
            &Some("15:00".to_string()),
            &None,
            &None,
            today,
            now,
        ));
    }

    #[test]
    fn train_on_next_day_is_not_mistakenly_passed() {
        let now = Local.with_ymd_and_hms(2024, 6, 7, 23, 50, 0).unwrap();
        let today = chrono::NaiveDate::from_ymd_opt(2024, 6, 8);
        // A train at 00:15 on the NEXT day is in the future, but raw HH:MM
        // comparison (00:15 < 23:50) would incorrectly mark it as passed.
        assert!(!compute_has_passed(
            &None,
            &Some("00:15".to_string()),
            &None,
            &None,
            today,
            now,
        ));
    }

    #[test]
    fn train_at_midnight_on_same_day_is_correctly_passed() {
        let now = Local.with_ymd_and_hms(2024, 6, 7, 23, 50, 0).unwrap();
        let today = chrono::NaiveDate::from_ymd_opt(2024, 6, 7);
        // A train at 00:15 on the SAME day is genuinely in the past.
        assert!(compute_has_passed(
            &None,
            &Some("00:15".to_string()),
            &None,
            &None,
            today,
            now,
        ));
    }

    #[test]
    fn estimated_time_takes_priority_for_passed_check() {
        let now = Local.with_ymd_and_hms(2024, 6, 7, 14, 5, 0).unwrap();
        let today = chrono::NaiveDate::from_ymd_opt(2024, 6, 7);
        // Scheduled departure is in the future, but estimated (actual) is in the past.
        assert!(compute_has_passed(
            &Some("14:02".to_string()),
            &Some("14:00".to_string()),
            &None,
            &None,
            today,
            now,
        ));
    }

    #[test]
    fn missing_time_defaults_to_not_passed() {
        let now = Local::now();
        let today = chrono::NaiveDate::from_ymd_opt(2024, 6, 7);
        assert!(!compute_has_passed(&None, &None, &None, &None, today, now));
    }
}
