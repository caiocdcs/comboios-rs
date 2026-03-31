use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StationBoardResponse {
    pub response: Vec<StationBoard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationBoard {
    pub station_id: String,
    pub station_name: String,
    pub trains: Vec<StationTimetable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationTimetable {
    pub train_number: u64,
    pub service_type: String,
    pub origin_station_name: String,
    pub origin_station_id: String,
    pub destination_station_name: String,
    pub destination_station_id: String,
    pub departure_time: Option<String>,
    pub arrival_time: Option<String>,
    pub platform: Option<String>,
    pub delay: Option<i32>,
    pub operator: String,
    pub has_passed: bool,
    pub is_departure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainEntry {
    #[serde(alias = "NComboio1")]
    pub train_number: u32,
    #[serde(alias = "NComboio2")]
    pub train_number_alt: u32,
    #[serde(alias = "NomeEstacaoOrigem")]
    pub origin_station_name: String,
    #[serde(alias = "NomeEstacaoDestino")]
    pub destination_station_name: String,
    #[serde(alias = "EstacaoOrigem")]
    pub origin_station_id: u32,
    #[serde(alias = "EstacaoDestino")]
    pub destination_station_id: u32,
    #[serde(alias = "DataHoraPartidaChegada")]
    pub time: String,
    #[serde(alias = "DataRealizacao")]
    pub date: String,
    #[serde(alias = "Observacoes")]
    pub observations: String,
    #[serde(alias = "TipoServico")]
    pub service_type: String,
    #[serde(alias = "ComboioPassou")]
    pub has_passed: bool,
    #[serde(alias = "Operador")]
    pub operator: String,
}

impl TrainEntry {
    pub fn delay_minutes(&self) -> Option<u32> {
        if self.observations.is_empty() {
            return None;
        }
        let re = regex::Regex::new(r"atraso\s+de\s+(\d+)\s+min").ok()?;
        re.captures(&self.observations.to_lowercase())
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse().ok())
    }
}

#[deprecated(
    since = "0.2.0",
    note = "Use StationBoard and StationTimetable instead"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timetable {
    pub delay: Option<i8>,
    #[serde(alias = "trainOrigin")]
    pub train_origin: super::station::Station,
    #[serde(alias = "trainDestination")]
    pub train_destination: super::station::Station,
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
