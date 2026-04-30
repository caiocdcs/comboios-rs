use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

/// Response wrapper returned by [`crate::Comboios::get_station_timetable`].
///
/// Contains one [`StationBoard`] entry per station returned by the CP API
/// (typically a single station matching the requested `station_id`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StationBoardResponse {
    /// Station boards; empty when no data is available for the requested
    /// station and date.
    pub response: Vec<StationBoard>,
}

/// Departure/arrival board for a single station on a specific date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationBoard {
    /// CP station identifier (e.g. `"94-31039"`).
    pub station_id: String,
    /// Human-readable station name.
    pub station_name: String,
    /// All train movements (arrivals and departures) at this station.
    pub trains: Vec<StationTimetable>,
}

/// A single train movement (one row on the departure/arrival board).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationTimetable {
    /// CP train number.
    pub train_number: u64,
    /// Service category: `"ALFA"`, `"IC"`, `"IR"`, `"REGIONAL"`, etc.
    pub service_type: String,
    /// Name of the origin station for this train service.
    pub origin_station_name: String,
    /// CP station identifier for the origin station.
    pub origin_station_id: String,
    /// Name of the destination station for this train service.
    pub destination_station_name: String,
    /// CP station identifier for the destination station.
    pub destination_station_id: String,
    /// Scheduled (or actual) departure time in `HH:MM` format.
    /// `None` for arrival-only movements.
    pub departure_time: Option<String>,
    /// Scheduled (or actual) arrival time in `HH:MM` format.
    /// `None` for departure-only movements.
    pub arrival_time: Option<String>,
    /// Assigned platform number, if available.
    pub platform: Option<String>,
    /// Delay in minutes; `None` when the train is on time or the value is
    /// not yet known.
    pub delay: Option<i32>,
    /// Operating company name.
    pub operator: String,
    /// `true` if the train has already passed through this station.
    pub has_passed: bool,
    /// `true` if this entry represents a departure; `false` for an arrival.
    pub is_departure: bool,
}

/// Raw train entry as returned by the CP API, before normalisation into
/// [`StationTimetable`].
///
/// Deserialization accepts the Portuguese field aliases used by the CP API
/// (e.g. `NComboio1`, `NomeEstacaoOrigem`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainEntry {
    /// Primary train number.
    #[serde(alias = "NComboio1")]
    pub train_number: u32,
    /// Alternate train number (e.g. for paired services).
    #[serde(alias = "NComboio2")]
    pub train_number_alt: u32,
    /// Name of the origin station.
    #[serde(alias = "NomeEstacaoOrigem")]
    pub origin_station_name: String,
    /// Name of the destination station.
    #[serde(alias = "NomeEstacaoDestino")]
    pub destination_station_name: String,
    /// Numeric CP identifier of the origin station.
    #[serde(alias = "EstacaoOrigem")]
    pub origin_station_id: u32,
    /// Numeric CP identifier of the destination station.
    #[serde(alias = "EstacaoDestino")]
    pub destination_station_id: u32,
    /// Departure or arrival time for this entry.
    #[serde(alias = "DataHoraPartidaChegada")]
    pub time: String,
    /// Service date in `YYYY-MM-DD` format.
    #[serde(alias = "DataRealizacao")]
    pub date: String,
    /// Free-text observations from CP (e.g. `"Atraso de 6 min."`).
    #[serde(alias = "Observacoes")]
    pub observations: String,
    /// Service category code (e.g. `"IC"`, `"IR"`, `"REGIONAL"`).
    #[serde(alias = "TipoServico")]
    pub service_type: String,
    /// `true` if the train has already passed through this station.
    #[serde(alias = "ComboioPassou")]
    pub has_passed: bool,
    /// Operating company name.
    #[serde(alias = "Operador")]
    pub operator: String,
}

static DELAY_RE: OnceLock<regex::Regex> = OnceLock::new();

impl TrainEntry {
    /// Parse the delay in minutes from the `observations` field.
    ///
    /// Returns `None` when `observations` is empty or does not contain a
    /// recognisable delay pattern (e.g. `"atraso de 6 min"`).
    ///
    /// # Panics
    ///
    /// Never panics — the regex pattern is a compile-time constant and is
    /// always valid.
    #[must_use]
    pub fn delay_minutes(&self) -> Option<u32> {
        if self.observations.is_empty() {
            return None;
        }
        let re = DELAY_RE.get_or_init(|| regex::Regex::new(r"atraso\s+de\s+(\d+)\s+min").unwrap());
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
