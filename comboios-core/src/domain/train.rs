use serde::{Deserialize, Serialize};

use super::station::Station;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stopover {
    pub station: Station,
    #[serde(alias = "departure")]
    pub departure_time: Option<String>,
    #[serde(alias = "arrival")]
    pub arrival_time: Option<String>,
    pub platform: Option<String>,
    pub latitude: String,
    pub longitude: String,
    pub delay: Option<i8>,
    pub eta: Option<String>,
    pub etd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Train {
    #[serde(alias = "trainNumber")]
    pub train_number: u32,
    pub delay: Option<i8>,
    pub occupancy: Option<u8>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub status: Option<String>,
    #[serde(alias = "trainStops")]
    pub stops: Vec<Stopover>,
}
