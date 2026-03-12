use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Station {
    #[serde(alias = "code")]
    pub id: String,
    #[serde(alias = "designation")]
    pub name: String,
}

/// Station board response from the API
#[derive(Debug, Deserialize, Clone)]
pub struct StationBoardResponse {
    pub data: Vec<StationBoard>,
}

/// Station board containing departures or arrivals
#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Clone, Deserialize)]
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
    pub fn delay_minutes(&self) -> Option<u32> {
        if self.observations.is_empty() {
            return None;
        }

        // Match patterns like "Circula com atraso de 49 min."
        let re = regex_lite::Regex::new(r"atraso\s+de\s+(\d+)\s+min").ok()?;
        re.captures(&self.observations.to_lowercase())
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse().ok())
    }

    /// Get display time with delay info
    pub fn display_time(&self) -> String {
        match self.delay_minutes() {
            Some(delay) if delay > 0 => format!("{} (+{} min)", self.time, delay),
            _ => self.time.clone(),
        }
    }
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
pub struct StationResponse {
    pub data: Vec<Station>,
}
