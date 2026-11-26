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
    #[serde(rename = "supression", default, deserialize_with = "de_supression")]
    pub supression: Option<String>,
}

/// CP sends `supression` either as a plain string or as
/// `{"code": "CANCELLED", "designation": "Suprimido"}`; keep the code.
fn de_supression<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Supression {
        Text(String),
        Coded { code: String },
    }
    Ok(Option::<Supression>::deserialize(d)?.map(|s| match s {
        Supression::Text(t) => t,
        Supression::Coded { code } => code,
    }))
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

#[cfg(test)]
mod tests {
    use super::{CpStationStop, CpTimetableResponse};

    fn stop_with(supression: &str) -> CpStationStop {
        let json = format!(
            r#"{{"trainNumber": 18418,
                "trainService": {{"code": "U", "designation": "Urbano"}},
                "trainOrigin": {{"code": "94-61101", "designation": "Sintra"}},
                "trainDestination": {{"code": "94-31039", "designation": "Lisboa Oriente"}},
                "arrivalTime": "08:06", "departureTime": "08:06",
                "platform": "3", "delay": -1{supression}}}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn supression_object_keeps_code() {
        let stop =
            stop_with(r#", "supression": {"code": "CANCELLED", "designation": "Suprimido"}"#);
        assert_eq!(stop.supression.as_deref(), Some("CANCELLED"));
    }

    #[test]
    fn supression_string_still_accepted() {
        let stop = stop_with(r#", "supression": "Supressão""#);
        assert_eq!(stop.supression.as_deref(), Some("Supressão"));
    }

    #[test]
    fn supression_null_or_missing_is_none() {
        assert_eq!(stop_with(r#", "supression": null"#).supression, None);
        assert_eq!(stop_with("").supression, None);
    }

    /// Real board for 94-60103 on 2026-11-26: 269 stops, 6 cancelled.
    #[test]
    fn board_with_cancelled_trains_decodes() {
        let board: CpTimetableResponse = serde_json::from_str(include_str!(
            "../../tests/fixtures/station_timetable_cancelled.json"
        ))
        .unwrap();
        assert_eq!(board.station_stops.len(), 269);
        let cancelled = board
            .station_stops
            .iter()
            .filter(|s| s.supression.as_deref() == Some("CANCELLED"))
            .count();
        assert_eq!(cancelled, 6);
    }
}
