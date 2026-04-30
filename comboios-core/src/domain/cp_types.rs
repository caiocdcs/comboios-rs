use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CpStation {
    pub code: String,
    pub designation: String,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub region: Option<String>,
    pub railways: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CpStationDetail {
    pub code: String,
    pub designation: String,
    pub latitude: f64,
    pub longitude: f64,
    pub train_line: Option<String>,
    pub mobility_access: Option<String>,
    pub services: Vec<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CpTimetableResponse {
    #[serde(rename = "stationStops")]
    pub station_stops: Vec<CpStationStop>,
    pub messages: Vec<CpMessage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CpStationStop {
    #[serde(rename = "trainNumber")]
    pub train_number: u64,
    #[serde(rename = "trainService")]
    pub train_service: CpServiceCode,
    #[serde(rename = "trainOrigin")]
    pub train_origin: CpStationSimple,
    #[serde(rename = "trainDestination")]
    pub train_destination: CpStationSimple,
    #[serde(rename = "arrivalTime")]
    pub arrival_time: Option<String>,
    #[serde(rename = "departureTime")]
    pub departure_time: Option<String>,
    pub platform: Option<String>,
    pub delay: Option<i32>,
    pub occupancy: Option<u32>,
    #[serde(rename = "ETA")]
    pub eta: Option<String>,
    #[serde(rename = "ETD")]
    pub etd: Option<String>,
    #[serde(rename = "supression")]
    pub supression: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CpServiceCode {
    pub code: String,
    pub designation: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CpStationSimple {
    pub code: String,
    pub designation: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CpMessage {
    #[serde(rename = "messageType")]
    pub message_type: Option<String>,
    #[serde(rename = "messageText")]
    pub message_text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CpTrainTimetable {
    #[serde(rename = "trainNumber")]
    pub train_number: u64,
    #[serde(rename = "serviceCode")]
    pub service_code: CpServiceCode,
    #[serde(rename = "lastStationCode")]
    pub last_station_code: Option<String>,
    pub delay: Option<i32>,
    pub occupancy: Option<u32>,
    #[serde(rename = "trainStops")]
    pub train_stops: Vec<CpTrainStop>,
    pub status: String,
    #[serde(rename = "hasDisruptions")]
    pub has_disruptions: Option<bool>,
    pub duration: Option<String>,
    pub messages: Vec<CpMessage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CpTrainStop {
    pub station: CpStationSimple,
    pub arrival: Option<String>,
    pub departure: Option<String>,
    pub platform: Option<String>,
    pub delay: Option<i32>,
    #[serde(rename = "ETA")]
    pub eta: Option<String>,
    #[serde(rename = "ETD")]
    pub etd: Option<String>,
}
