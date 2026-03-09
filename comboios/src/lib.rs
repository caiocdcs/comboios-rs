pub mod client;
pub mod domain;
pub mod error;
pub mod query_builder;

// Re-export the main API client and configuration
pub use client::{ComboiosApi, ApiConfig, RetryConfig};

// Re-export common types at crate root for easier imports
pub use domain::station::{Station, StationResponse};
pub use domain::station_timetable::Timetable;
pub use domain::train::{Stopover, Train};
pub use error::CoreError;
pub use query_builder::{StationQuery, TimetableFilter};
