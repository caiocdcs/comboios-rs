//! Service alert domain types
//!
//! This module provides types for representing service alerts and disruptions
//! from various sources including Infraestruturas de Portugal and Comboios de Portugal.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Service alert/disruption information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAlert {
    /// Unique identifier for the alert
    pub id: String,
    /// Brief title of the alert
    pub title: String,
    /// Detailed description of the alert
    pub description: String,
    /// Severity level of the alert
    pub severity: AlertSeverity,
    /// Category of the alert
    pub category: AlertCategory,
    /// Lines affected by this alert (e.g., "IC", "IR", "Regional")
    pub affected_lines: Vec<String>,
    /// Stations affected by this alert
    pub affected_stations: Vec<String>,
    /// When the alert starts (if known)
    pub start_time: Option<DateTime<Utc>>,
    /// When the alert ends (if known)
    pub end_time: Option<DateTime<Utc>>,
    /// When this alert was last updated
    pub last_updated: DateTime<Utc>,
    /// URL to more detailed information (if available)
    pub url: Option<String>,
    /// Source of this alert
    pub source: AlertSource,
}

/// Severity levels for service alerts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    /// Informational alert (service note, schedule change)
    Info,
    /// Warning alert (potential delays, minor disruptions)
    Warning,
    /// Critical alert (major disruptions, cancellations)
    Critical,
}

/// Categories of service alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCategory {
    /// Infrastructure works (construction, maintenance)
    Infrastructure,
    /// Schedule changes (timetable modifications)
    ScheduleChange,
    /// Weather-related issues
    Weather,
    /// Technical issues (equipment failures)
    TechnicalIssue,
    /// Special events affecting service
    SpecialEvent,
    /// Other types of alerts
    Other,
}

/// Sources of service alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSource {
    /// Infraestruturas de Portugal
    InfraestruturasPortugal,
    /// Comboios de Portugal
    ComboiosPortugal,
    /// User-reported information
    UserReported,
}

impl std::fmt::Display for AlertSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertSource::InfraestruturasPortugal => write!(f, "InfraestruturasPortugal"),
            AlertSource::ComboiosPortugal => write!(f, "ComboiosPortugal"),
            AlertSource::UserReported => write!(f, "UserReported"),
        }
    }
}

/// Response wrapper for alert queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertResponse {
    pub alerts: Vec<ServiceAlert>,
}

impl ServiceAlert {
    /// Check if this alert affects a specific line
    pub fn affects_line(&self, line: &str) -> bool {
        self.affected_lines.is_empty()
            || self
                .affected_lines
                .iter()
                .any(|l| l.to_lowercase() == line.to_lowercase())
    }

    /// Check if this alert affects a specific station
    pub fn affects_station(&self, station: &str) -> bool {
        self.affected_stations.is_empty()
            || self
                .affected_stations
                .iter()
                .any(|s| s.to_lowercase() == station.to_lowercase())
    }

    /// Check if this alert is currently active
    pub fn is_active(&self) -> bool {
        let now = Utc::now();

        match (&self.start_time, &self.end_time) {
            (Some(start), Some(end)) => &now >= start && &now <= end,
            (Some(start), None) => &now >= start,
            (None, Some(end)) => &now <= end,
            (None, None) => true,
        }
    }
}
