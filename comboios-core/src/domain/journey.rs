//! Train journey domain types.
//!
//! Provides types for tracking live train journeys, including per-stop arrival
//! and departure times, real-time delay data, and overall journey status.
//! Journey data is sourced from the CP API (primary) with IP as a fallback.

use serde::{Deserialize, Serialize};

use super::station::Station;

/// Complete information for a single train journey, including all stops and
/// real-time status.
///
/// Obtain a value of this type via [`crate::Comboios::get_train_journey`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainJourney {
    /// CP train number (e.g. `"120"`).
    pub train_number: String,
    /// Service category: `"ALFA"`, `"IC"`, `"IR"`, `"REGIONAL"`, etc.
    pub service_type: String,
    /// First stop (origin station) of this journey.
    pub origin: Station,
    /// Last stop (destination station) of this journey.
    pub destination: Station,
    /// Ordered list of all stops from origin to destination.
    pub stops: Vec<JourneyStop>,
    /// Overall real-time status of the journey.
    pub status: JourneyStatus,
    /// Net delay in minutes at the time the data was fetched, if known.
    pub delay_minutes: Option<i32>,
    /// Operating company name.
    pub operator: String,
    /// Free-text status message from CP (e.g. `"Circula com atraso de 6 min."`).
    /// `None` when no observations are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<String>,
    /// Scheduled total journey duration in `HH:MM:SS` format, if provided by
    /// the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

/// Real-time information for one stop within a [`TrainJourney`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyStop {
    /// Station at this stop.
    pub station: Station,
    /// Timetabled arrival time at this station (`HH:MM` format).
    pub scheduled_arrival: String,
    /// Actual arrival time once the train has arrived; `None` for future stops.
    pub actual_arrival: Option<String>,
    /// Timetabled departure time from this station (`HH:MM` format).
    pub scheduled_departure: String,
    /// Actual departure time once the train has left; `None` for future stops.
    pub actual_departure: Option<String>,
    /// Platform number assigned at this stop, if known.
    pub platform: Option<String>,
    /// Real-time status of the train at this stop.
    pub status: StopStatus,
    /// Delay in minutes at this stop; `None` when on time or unknown.
    pub delay_minutes: Option<i32>,
    /// 1-based position of this stop in the journey.
    pub stop_number: usize,
    /// `true` once the train has passed through this station.
    /// `None` when the information is not available from the data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_passed: Option<bool>,
    /// Predicted arrival/departure time sourced from IP API observations.
    /// `None` when IP data is not available or not applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_time: Option<String>,
}

/// Overall real-time status of a [`TrainJourney`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JourneyStatus {
    /// Train has not yet departed from its origin station.
    Scheduled,
    /// Train is currently between its origin and destination.
    InProgress,
    /// Train has arrived at its destination station.
    Completed,
    /// Train is running but behind schedule.
    Delayed,
    /// Train service has been cancelled for this date.
    Cancelled,
    /// Status could not be determined from available data.
    Unknown,
}

/// Real-time status of the train at a specific [`JourneyStop`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StopStatus {
    /// Train has not yet reached this stop.
    Scheduled,
    /// Train is currently at this stop (boarding/alighting in progress).
    AtStop,
    /// Train has departed this stop.
    Departed,
    /// Train passed through without stopping (non-service stop).
    Passed,
    /// This stop has been cancelled for this journey.
    Cancelled,
    /// Status could not be determined from available data.
    Unknown,
}

impl JourneyStop {
    /// Returns `true` if the train has arrived at (or departed from) this stop.
    #[must_use]
    pub fn has_arrived(&self) -> bool {
        matches!(self.status, StopStatus::AtStop | StopStatus::Departed)
    }

    /// Returns `true` if the train has already departed this stop.
    #[must_use]
    pub fn has_departed(&self) -> bool {
        matches!(self.status, StopStatus::Departed)
    }

    /// Returns `true` if the train has not yet reached this stop.
    #[must_use]
    pub fn is_future(&self) -> bool {
        matches!(self.status, StopStatus::Scheduled)
    }

    /// Returns the actual arrival time when available, falling back to the
    /// scheduled arrival time.
    #[must_use]
    pub fn display_arrival(&self) -> &str {
        self.actual_arrival
            .as_deref()
            .unwrap_or(&self.scheduled_arrival)
    }

    /// Returns the actual departure time when available, falling back to the
    /// scheduled departure time.
    #[must_use]
    pub fn display_departure(&self) -> &str {
        self.actual_departure
            .as_deref()
            .unwrap_or(&self.scheduled_departure)
    }
}

impl TrainJourney {
    /// Returns the stop where the train currently is, or the next upcoming stop
    /// if the train is between stations.
    ///
    /// Prefers a stop with [`StopStatus::AtStop`]; falls back to the first
    /// [`StopStatus::Scheduled`] stop. Returns `None` if the journey is
    /// completed.
    #[must_use]
    pub fn current_stop(&self) -> Option<&JourneyStop> {
        self.stops
            .iter()
            .find(|s| s.status == StopStatus::AtStop)
            .or_else(|| {
                self.stops
                    .iter()
                    .find(|s| s.status == StopStatus::Scheduled)
            })
    }

    /// Returns up to `count` upcoming stops (including any stop the train is
    /// currently at).
    #[must_use]
    pub fn upcoming_stops(&self, count: usize) -> Vec<&JourneyStop> {
        self.stops
            .iter()
            .filter(|s| s.is_future() || s.status == StopStatus::AtStop)
            .take(count)
            .collect()
    }

    /// Returns all stops from which the train has already departed.
    #[must_use]
    pub fn completed_stops(&self) -> Vec<&JourneyStop> {
        self.stops.iter().filter(|s| s.has_departed()).collect()
    }

    /// Returns the journey progress as a percentage (`0`–`100`) based on the
    /// number of departed stops relative to the total number of stops.
    #[must_use]
    pub fn progress_percent(&self) -> u8 {
        let total = self.stops.len();
        if total == 0 {
            return 0;
        }

        let completed = self.stops.iter().filter(|s| s.has_departed()).count();
        u8::try_from((completed * 100) / total).unwrap_or(100)
    }

    /// Returns the actual arrival time at the destination if the train has
    /// already arrived, or `None` while the journey is still in progress.
    #[must_use]
    pub fn estimated_arrival(&self) -> Option<&str> {
        self.stops.last().and_then(|s| s.actual_arrival.as_deref())
    }
}

/// Request for journey information
#[derive(Debug, Clone)]
pub struct JourneyRequest {
    pub train_number: String,
    pub date: Option<String>, // Format: YYYY-MM-DD
}

/// Response wrapper for journey queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyResponse {
    pub journey: Option<TrainJourney>,
}
