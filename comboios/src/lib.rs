pub mod client;
pub mod domain;
pub mod error;

// Re-export the main API client for convenience
pub use client::ComboiosApi;

// Re-export common types
pub use domain::{station::StationResponse, station_timetable::Timetable, train::Train};
pub use error::CoreError;
