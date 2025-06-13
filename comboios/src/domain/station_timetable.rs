use serde::{Deserialize, Serialize};

use super::station::Station;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timetable {
    delay: Option<i8>,
    #[serde(alias = "trainOrigin")]
    train_origin: Station,
    #[serde(alias = "trainDestination")]
    train_destination: Station,
    #[serde(alias = "departureTime")]
    departure_time: Option<String>,
    #[serde(alias = "arrivalTime")]
    arrival_time: Option<String>,
    #[serde(alias = "trainNumber")]
    train_number: u32,
    platform: Option<String>,
    occupancy: Option<u8>,
    eta: Option<String>,
    etd: Option<String>,
}
