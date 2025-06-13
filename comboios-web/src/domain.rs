use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Station {
    #[serde(alias = "code")]
    pub id: String,
    #[serde(alias = "designation")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct Stopover {
    pub station: Station,
    #[serde(alias = "departure_time")]
    pub departure_time: Option<String>,
    #[serde(alias = "arrival_time")]
    pub arrival_time: Option<String>,
    pub platform: Option<String>,
    pub latitude: String,
    pub longitude: String,
    pub delay: Option<i8>,
    pub eta: Option<String>,
    pub etd: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct StopoverResponse {
    pub data: Vec<Timetable>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrainDetails {
    #[serde(alias = "train_number")]
    pub id: u32,
    pub delay: Option<i8>,
    pub occupancy: Option<u8>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub status: Option<String>,
    #[serde(alias = "stops")]
    pub stops: Option<Vec<Stopover>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StationResponse {
    pub data: Vec<Station>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrainDetailsResponse {
    pub data: TrainDetails,
}
