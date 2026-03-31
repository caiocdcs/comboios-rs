//! Train journey domain types
//!
//! This module provides types for tracking train journeys with stops,
//! which is available from the CP API but not the IP API.

use serde::{Deserialize, Serialize};

use super::station::Station;

/// Complete train journey information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainJourney {
    /// Train number/identifier
    pub train_number: String,
    /// Service type (ALFA, IC, IR, REGIONAL, etc.)
    pub service_type: String,
    /// Origin station
    pub origin: Station,
    /// Destination station
    pub destination: Station,
    /// All stops along the journey
    pub stops: Vec<JourneyStop>,
    /// Current status of the journey
    pub status: JourneyStatus,
    /// Overall delay in minutes (if any)
    pub delay_minutes: Option<i32>,
    /// Operator name
    pub operator: String,
    /// Train status observations (e.g., "Circula com atraso de 6 min.")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<String>,
    /// Journey duration (e.g., "02:30:00")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

/// Individual stop in a train journey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyStop {
    /// Station information
    pub station: Station,
    /// Scheduled arrival time
    pub scheduled_arrival: String,
    /// Actual arrival time (if arrived)
    pub actual_arrival: Option<String>,
    /// Scheduled departure time
    pub scheduled_departure: String,
    /// Actual departure time (if departed)
    pub actual_departure: Option<String>,
    /// Platform number
    pub platform: Option<String>,
    /// Stop status
    pub status: StopStatus,
    /// Delay at this stop (minutes)
    pub delay_minutes: Option<i32>,
    /// Stop number in journey (1-based)
    pub stop_number: usize,
    /// Whether train has passed this station
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_passed: Option<bool>,
    /// Predicted time (from IP API observations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_time: Option<String>,
}

/// Journey status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JourneyStatus {
    /// Train not yet departed from origin
    Scheduled,
    /// Train is en route
    InProgress,
    /// Train has arrived at destination
    Completed,
    /// Train is delayed
    Delayed,
    /// Train is cancelled
    Cancelled,
    /// Unknown status
    Unknown,
}

/// Stop status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StopStatus {
    /// Stop scheduled but train hasn't arrived
    Scheduled,
    /// Train currently at this stop
    AtStop,
    /// Train has departed this stop
    Departed,
    /// Train passed without stopping
    Passed,
    /// Stop cancelled
    Cancelled,
    /// Unknown status
    Unknown,
}

impl JourneyStop {
    /// Check if train has arrived at this stop
    pub fn has_arrived(&self) -> bool {
        matches!(self.status, StopStatus::AtStop | StopStatus::Departed)
    }

    /// Check if train has departed this stop
    pub fn has_departed(&self) -> bool {
        matches!(self.status, StopStatus::Departed)
    }

    /// Check if this stop is in the future
    pub fn is_future(&self) -> bool {
        matches!(self.status, StopStatus::Scheduled)
    }

    /// Get display time (actual if available, otherwise scheduled)
    pub fn display_arrival(&self) -> &str {
        self.actual_arrival
            .as_deref()
            .unwrap_or(&self.scheduled_arrival)
    }

    /// Get display departure time
    pub fn display_departure(&self) -> &str {
        self.actual_departure
            .as_deref()
            .unwrap_or(&self.scheduled_departure)
    }
}

impl TrainJourney {
    /// Get current/next stop
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

    /// Get next N upcoming stops
    pub fn upcoming_stops(&self, count: usize) -> Vec<&JourneyStop> {
        self.stops
            .iter()
            .filter(|s| s.is_future() || s.status == StopStatus::AtStop)
            .take(count)
            .collect()
    }

    /// Get stops that have been completed
    pub fn completed_stops(&self) -> Vec<&JourneyStop> {
        self.stops.iter().filter(|s| s.has_departed()).collect()
    }

    /// Calculate progress percentage
    pub fn progress_percent(&self) -> u8 {
        let total = self.stops.len();
        if total == 0 {
            return 0;
        }

        let completed = self.stops.iter().filter(|s| s.has_departed()).count();
        ((completed as f32 / total as f32) * 100.0) as u8
    }

    /// Estimated arrival at destination
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
