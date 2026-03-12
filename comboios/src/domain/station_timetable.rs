use serde::{Deserialize, Serialize};

/// Response from the station timetable API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationBoardResponse {
    pub response: Vec<StationBoard>,
}

/// Station board containing departures or arrivals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationBoard {
    #[serde(alias = "NodeID")]
    pub station_id: u32,
    #[serde(alias = "NomeEstacao")]
    pub station_name: String,
    #[serde(alias = "TipoPedido")]
    pub request_type: u8,
    #[serde(alias = "NodesComboioTabelsPartidasChegadas")]
    pub trains: Vec<TrainEntry>,
}

/// Individual train entry in the station board
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
    /// Parse delay in minutes from observations string
    /// Returns Some(minutes) if delay is mentioned, None otherwise
    pub fn delay_minutes(&self) -> Option<u32> {
        if self.observations.is_empty() {
            return None;
        }

        // Match patterns like "Circula com atraso de 49 min."
        // or "Atraso de 5 min."
        let re = regex::Regex::new(r"atraso\s+de\s+(\d+)\s+min").ok()?;
        re.captures(&self.observations.to_lowercase())
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse().ok())
    }
}

/// Deprecated: Old timetable structure for CP API
#[deprecated(since = "0.2.0", note = "Use StationBoard and TrainEntry instead")]
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
