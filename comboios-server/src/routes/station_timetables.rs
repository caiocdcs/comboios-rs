use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use chrono::{DateTime, NaiveDate};
use chrono::{Duration, TimeZone, Utc};
use chrono_tz::Europe::Lisbon;
use chrono_tz::Tz;
use comboios_core::domain::station_timetable::{StationBoard, StationTimetable};

use crate::{
    domain::{AppResponse, AppState},
    error::AppError,
};

/// How far back in time to ask CP for train movements. CP filters `start` by
/// *scheduled* time, so delayed trains whose scheduled time is already in the
/// past would vanish from the board without this buffer. They are filtered out
/// locally afterwards using their real effective time.
const LOOKBACK: Duration = Duration::minutes(60);

/// # Errors
///
/// Returns [`AppError`] if the CP API call fails.
#[tracing::instrument(skip(state))]
pub async fn station_timetables(
    State(state): State<Arc<AppState>>,
    Path(station_id): Path<String>,
) -> Result<Json<AppResponse<Vec<StationBoard>>>, AppError> {
    tracing::info!("Finding timetable for station {}", station_id);

    // CP times are Portugal-local; always compute "now" in Europe/Lisbon so
    // boards are identical regardless of the host timezone (containers run in
    // UTC and would otherwise show trains that already departed).
    let now = Utc::now().with_timezone(&Lisbon);
    let date = now.format("%Y-%m-%d").to_string();
    let today = now.date_naive();

    // Ask CP from LOOKBACK minutes ago so delayed trains whose scheduled time
    // already passed are still returned; they are filtered locally afterwards.
    // When the lookback crosses midnight the previous-day window cannot be
    // expressed with a same-day HH:MM start, so fetch the full day instead.
    let lookback = now - LOOKBACK;
    let start_time = if lookback.date_naive() == today {
        Some(lookback.format("%H:%M").to_string())
    } else {
        None
    };

    let mut boards = state
        .api
        .get_station_timetable(&station_id, &date, start_time.as_deref())
        .await?;

    for board in &mut boards.response {
        if board.station_name.is_empty()
            && let Ok(names) = state.station_names.read()
            && let Some(name) = names.get(&board.station_id)
        {
            board.station_name = name.clone();
        }

        retain_upcoming(&mut board.trains, today, now);
    }

    Ok(Json(AppResponse {
        data: boards.response,
    }))
}

/// Drop trains that have already passed the station (by effective time) and
/// mark the rest as not passed.
fn retain_upcoming(trains: &mut Vec<StationTimetable>, today: NaiveDate, now: DateTime<Tz>) {
    trains.retain(|t| {
        !compute_has_passed(
            &t.estimated_departure,
            &t.departure_time,
            &t.estimated_arrival,
            &t.arrival_time,
            t.delay,
            today,
            now,
        )
    });
    for t in trains.iter_mut() {
        t.has_passed = false;
    }
}

/// Determine whether a train has already passed the station by comparing its
/// effective time against `now`.
///
/// Effective time is the first available of: estimated departure, scheduled
/// departure plus delay, estimated arrival, scheduled arrival plus delay.
/// Trains without any usable time are treated as upcoming.
fn compute_has_passed(
    estimated_departure: &Option<String>,
    departure_time: &Option<String>,
    estimated_arrival: &Option<String>,
    arrival_time: &Option<String>,
    delay: Option<i32>,
    today: NaiveDate,
    now: DateTime<Tz>,
) -> bool {
    let delay = Duration::minutes(i64::from(delay.unwrap_or(0)));

    let candidates = [
        (estimated_departure, Duration::zero()),
        (departure_time, delay),
        (estimated_arrival, Duration::zero()),
        (arrival_time, delay),
    ];

    let effective = candidates.iter().find_map(|(time, adjust)| {
        let t = time
            .as_deref()
            .and_then(|t| chrono::NaiveTime::parse_from_str(t, "%H:%M").ok())?;
        let dt = Lisbon.from_local_datetime(&today.and_time(t)).single()?;
        Some(dt + *adjust)
    });

    match effective {
        Some(dt) => dt <= now,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn now_at(h: u32, m: u32) -> DateTime<Tz> {
        Lisbon.with_ymd_and_hms(2024, 6, 7, h, m, 0).unwrap()
    }

    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 6, 7).unwrap()
    }

    #[test]
    fn train_in_the_past_is_passed() {
        assert!(compute_has_passed(
            &None,
            &Some("13:30".to_string()),
            &None,
            &None,
            None,
            today(),
            now_at(14, 0),
        ));
    }

    #[test]
    fn train_in_the_future_is_not_passed() {
        assert!(!compute_has_passed(
            &None,
            &Some("15:00".to_string()),
            &None,
            &None,
            None,
            today(),
            now_at(14, 0),
        ));
    }

    #[test]
    fn estimated_time_takes_priority_for_passed_check() {
        // Scheduled departure is in the future, but estimated (actual) is in the past.
        assert!(compute_has_passed(
            &Some("14:02".to_string()),
            &Some("14:00".to_string()),
            &None,
            &None,
            None,
            today(),
            now_at(14, 5),
        ));
    }

    #[test]
    fn missing_time_defaults_to_not_passed() {
        assert!(!compute_has_passed(
            &None,
            &None,
            &None,
            &None,
            None,
            today(),
            now_at(14, 0)
        ));
    }

    #[test]
    fn delayed_train_scheduled_in_past_but_still_upcoming_is_not_passed() {
        // Scheduled 13:30 with +45min delay -> effective 14:15, still upcoming at 14:00.
        assert!(!compute_has_passed(
            &None,
            &Some("13:30".to_string()),
            &None,
            &None,
            Some(45),
            today(),
            now_at(14, 0),
        ));
    }

    #[test]
    fn delayed_train_whose_effective_time_passed_is_passed() {
        // Scheduled 13:30 with +20min delay -> effective 13:50, past by 14:00.
        assert!(compute_has_passed(
            &None,
            &Some("13:30".to_string()),
            &None,
            &None,
            Some(20),
            today(),
            now_at(14, 0),
        ));
    }

    #[test]
    fn arrival_only_train_uses_arrival_plus_delay() {
        assert!(compute_has_passed(
            &None,
            &None,
            &None,
            &Some("13:50".to_string()),
            Some(5),
            today(),
            now_at(14, 0),
        ));
    }

    #[test]
    fn train_on_next_day_is_not_mistakenly_passed() {
        let tomorrow = NaiveDate::from_ymd_opt(2024, 6, 8).unwrap();
        // A train at 00:15 on the NEXT day is in the future, but raw HH:MM
        // comparison (00:15 < 23:50) would incorrectly mark it as passed.
        assert!(!compute_has_passed(
            &None,
            &Some("00:15".to_string()),
            &None,
            &None,
            None,
            tomorrow,
            now_at(23, 50),
        ));
    }

    #[test]
    fn train_at_midnight_on_same_day_is_correctly_passed() {
        // A train at 00:15 on the SAME day is genuinely in the past.
        assert!(compute_has_passed(
            &None,
            &Some("00:15".to_string()),
            &None,
            &None,
            None,
            today(),
            now_at(23, 50),
        ));
    }

    #[test]
    fn negative_delay_moves_effective_time_earlier() {
        // Early departure: scheduled 14:30, -10min -> effective 14:20, passed by 14:30.
        assert!(compute_has_passed(
            &None,
            &Some("14:30".to_string()),
            &None,
            &None,
            Some(-10),
            today(),
            now_at(14, 30),
        ));
    }

    fn make_train(departure_time: Option<&str>, delay: Option<i32>) -> StationTimetable {
        StationTimetable {
            train_number: 120,
            service_type: "IC|Intercidades".to_string(),
            origin_station_name: "Lisboa".to_string(),
            origin_station_id: "94-001".to_string(),
            destination_station_name: "Porto".to_string(),
            destination_station_id: "94-002".to_string(),
            departure_time: departure_time.map(str::to_string),
            arrival_time: None,
            platform: Some("3".to_string()),
            delay,
            estimated_departure: None,
            estimated_arrival: None,
            observations: None,
            operator: "CP".to_string(),
            has_passed: true,
            is_departure: true,
        }
    }

    #[test]
    fn retain_upcoming_drops_passed_and_clears_flag() {
        let mut trains = vec![
            make_train(Some("13:30"), None),     // passed
            make_train(Some("13:40"), Some(45)), // delayed, effective 14:25 -> upcoming
            make_train(Some("15:00"), None),     // upcoming
        ];

        retain_upcoming(&mut trains, today(), now_at(14, 0));

        assert_eq!(trains.len(), 2);
        assert_eq!(trains[0].departure_time.as_deref(), Some("13:40"));
        assert_eq!(trains[1].departure_time.as_deref(), Some("15:00"));
        assert!(trains.iter().all(|t| !t.has_passed));
    }
}
