pub mod client;
pub mod domain;
pub mod error;
pub mod providers;
pub mod query_builder;

// Re-export the main API client and configuration
pub use client::{ApiConfig, ComboiosApi, RetryConfig};

// Re-export providers
pub use providers::{
    ComboiosClient, CpConfig, CpProvider, DataProvider, IpProvider, UnifiedProvider, to_cp_id,
    to_ip_id,
};

// Re-export journey types
pub use domain::journey::{
    JourneyRequest, JourneyResponse, JourneyStatus, JourneyStop, StopStatus, TrainJourney,
};

// Re-export common types at crate root for easier imports
pub use domain::station::{Station, StationResponse};
pub use domain::station_timetable::{StationBoard, StationBoardResponse, TrainEntry};
pub use domain::train::{Stopover, Train};
pub use error::CoreError;
pub use query_builder::{StationQuery, TrainEntryFilter};
