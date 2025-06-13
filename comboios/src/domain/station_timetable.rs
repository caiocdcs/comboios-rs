use serde::{Deserialize, Serialize};

use super::station::Station;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timetable {
    pub delay: Option<i8>,
    #[serde(alias = "trainOrigin")]
    pub train_origin: Station,
    #[serde(alias = "trainDestination")]
    pub train_destination: Station,
    #[serde(alias = "departureTime")]
    pub departure_time: Option<String>,
    #[serde(alias = "arrivalTime")]
    pub arrival_time: Option<String>,
    #[serde(alias = "trainNumber")]
    pub train_number: u32,
    pub platform: Option<String>,
    pub occupancy: Option<u8>,
    pub eta: Option<String>,
    pub etd: Option<String>,
}
