//! # comboios-core
//!
//! Unofficial Rust client for [Comboios de Portugal (CP)](https://www.cp.pt) APIs.
//!
//! Provides real-time train data: station search, departure/arrival boards,
//! and live train journey tracking with stop-by-stop status.
//!
//! ## Data Sources
//!
//! - **CP API Gateway** (`api-gateway.cp.pt`) — primary source for timetables and
//!   train journeys. Credentials are fetched automatically from `cp.pt` on startup.
//! - **Infraestruturas de Portugal** (`infraestruturasdeportugal.pt`) — fallback for
//!   train journey data when CP is unavailable.
//!
//! ## Quick Start
//!
//! ```no_run
//! use comboios_core::Comboios;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), comboios_core::Error> {
//!     // Fetches live CP credentials from cp.pt on startup
//!     let client = Comboios::new().await?;
//!
//!     // Search for stations
//!     let stations = client.search_stations("Lisboa").await?;
//!     for station in &stations.response {
//!         println!("{}: {}", station.code, station.designation);
//!     }
//!
//!     // Get live departure board
//!     let today = chrono::Local::now().format("%Y-%m-%d").to_string();
//!     let board = client.get_station_timetable("94-31039", &today, None).await?;
//!
//!     // Track a train
//!     let journey = client.get_train_journey("120", &today).await?;
//!     println!("Train {} is {:?}", journey.train_number, journey.status);
//!
//!     Ok(())
//! }
//! ```

pub mod adapters;
pub mod domain;
pub mod error;
pub mod query_builder;

pub(crate) mod constants;

pub use client::Comboios;
pub use error::CoreError as Error;

mod client;
