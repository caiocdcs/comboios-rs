use serde::{Deserialize, Serialize};

use super::station::Station;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stopover {
    station: Station,
    #[serde(alias = "departure")]
    departure_time: Option<String>,
    #[serde(alias = "arrival")]
    arrival_time: Option<String>,
    platform: Option<String>,
    latitude: String,
    longitude: String,
    delay: Option<i8>,
    eta: Option<String>,
    etd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Train {
    #[serde(alias = "trainNumber")]
    train_number: u32,
    delay: Option<i8>,
    occupancy: Option<u8>,
    latitude: Option<String>,
    longitude: Option<String>,
    status: Option<String>,
    #[serde(alias = "trainStops")]
    stops: Vec<Stopover>,
}
