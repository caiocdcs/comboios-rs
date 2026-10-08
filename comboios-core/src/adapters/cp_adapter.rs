use std::fmt::Write as _;

use reqwest::Client;

use crate::domain::cp_types::{CpStation, CpStationStop, CpTimetableResponse, CpTrainTimetable};
use crate::domain::{
    journey::TrainJourney,
    station::Station as DomainStation,
    station::StationResponse,
    station_timetable::{StationBoard, StationBoardResponse, StationTimetable},
};
use crate::error::CoreError;

type Result<T> = std::result::Result<T, CoreError>;

use crate::constants::{CP_BASE_URL, USER_AGENT};

#[derive(Clone)]
pub struct CpAdapter {
    client: Client,
    base_url: String,
    api_key: String,
    connect_id: String,
    connect_secret: String,
}

impl CpAdapter {
    pub fn new(api_key: String, connect_id: String, connect_secret: String) -> Self {
        Self::with_base_url(CP_BASE_URL, api_key, connect_id, connect_secret)
    }

    pub fn with_base_url(
        base_url: &str,
        api_key: String,
        connect_id: String,
        connect_secret: String,
    ) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_key,
            connect_id,
            connect_secret,
        }
    }

    pub async fn search_stations(&self, query: &str) -> Result<StationResponse> {
        if query.trim().is_empty() {
            return Err(CoreError::InvalidInput(
                "station search query must not be empty".to_string(),
            ));
        }

        let stations = self.fetch_stations().await?;

        Ok(StationResponse {
            response: Self::filter_stations(&stations, query),
        })
    }

    /// Every station CP knows about, e.g. to resolve station codes to names.
    pub async fn list_stations(&self) -> Result<StationResponse> {
        let stations = self.fetch_stations().await?;

        Ok(StationResponse {
            response: stations
                .into_iter()
                .map(|s| DomainStation {
                    code: s.code,
                    designation: s.designation,
                })
                .collect(),
        })
    }

    async fn fetch_stations(&self) -> Result<Vec<CpStation>> {
        let url = format!("{}/services/travel-api/stations", self.base_url);
        self.get(&url).await
    }

    /// Keep stations whose name contains every word of `query`.
    ///
    /// CP station names carry no diacritics ("Cais do Sodre", "Porto Campanha"),
    /// so both sides are folded to lowercase ASCII before comparing: otherwise
    /// "sodré" or "São Bento" typed on a Portuguese keyboard match nothing.
    pub(crate) fn filter_stations(stations: &[CpStation], query: &str) -> Vec<DomainStation> {
        let words: Vec<String> = query.split_whitespace().map(fold).collect();

        stations
            .iter()
            .filter(|s| {
                let name = fold(&s.designation);
                words.iter().all(|w| name.contains(w.as_str()))
            })
            .map(|s| DomainStation {
                code: s.code.clone(),
                designation: s.designation.clone(),
            })
            .collect()
    }

    pub async fn get_station_timetable(
        &self,
        station_id: &str,
        date: &str,
        start_time: Option<&str>,
    ) -> Result<StationBoardResponse> {
        let mut url = format!(
            "{}/services/travel-api/stations/{}/timetable/{}",
            self.base_url, station_id, date
        );
        if let Some(start) = start_time {
            write!(url, "?start={start}").expect("writing to String never fails");
        }

        let response: CpTimetableResponse = self.get(&url).await?;

        let board = Self::convert_timetable_to_board(station_id, &response);
        Ok(StationBoardResponse {
            response: vec![board],
        })
    }

    pub async fn get_train_journey(&self, train_number: &str, date: &str) -> Result<TrainJourney> {
        let url = format!(
            "{}/services/travel-api/trains/{}/timetable/{}",
            self.base_url, train_number, date
        );
        let timetable: CpTrainTimetable = self.get(&url).await?;
        Ok(timetable.to_train_journey())
    }

    pub(crate) fn convert_timetable_to_board(
        station_id: &str,
        response: &CpTimetableResponse,
    ) -> StationBoard {
        // CP API does not include the queried station name in the timetable
        // response. The name is populated server-side from a cached station list.
        let station_name = String::new();

        let trains: Vec<StationTimetable> = response
            .station_stops
            .iter()
            .map(|stop| Self::convert_stop_to_timetable(stop, station_id))
            .collect();

        StationBoard {
            station_id: station_id.to_string(),
            station_name,
            trains,
        }
    }

    pub(crate) fn convert_stop_to_timetable(
        stop: &CpStationStop,
        _station_id: &str,
    ) -> StationTimetable {
        let is_departure = stop.departure_time.is_some();

        StationTimetable {
            train_number: stop.train_number,
            service_type: format!(
                "{}|{}",
                stop.train_service.code, stop.train_service.designation
            ),
            origin_station_name: stop.train_origin.designation.clone(),
            origin_station_id: stop.train_origin.code.clone(),
            destination_station_name: stop.train_destination.designation.clone(),
            destination_station_id: stop.train_destination.code.clone(),
            departure_time: stop.departure_time.clone(),
            arrival_time: stop.arrival_time.clone(),
            estimated_departure: stop.etd.clone(),
            estimated_arrival: stop.eta.clone(),
            platform: stop.platform.clone(),
            delay: stop.delay,
            observations: stop.supression.clone(),
            operator: "CP".to_string(),
            has_passed: false,
            is_departure,
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_secs(30))
            .header("User-Agent", USER_AGENT)
            .header("Accept", "application/json")
            .header("Origin", "https://www.cp.pt")
            .header("Referer", "https://www.cp.pt/")
            .header("x-api-key", &self.api_key)
            .header("x-cp-connect-id", &self.connect_id)
            .header("x-cp-connect-secret", &self.connect_secret)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(CoreError::ApiError {
                status: status.as_u16(),
                message: text,
            });
        }

        let data = response.json::<T>().await?;
        Ok(data)
    }
}

/// Lowercase and strip Portuguese diacritics, for accent-insensitive matching
fn fold(s: &str) -> String {
    s.chars()
        .flat_map(char::to_lowercase)
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            c => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::adapters::cp_adapter::CpAdapter;
    use crate::domain::cp_types::{
        CpServiceCode, CpStation, CpStationSimple, CpStationStop, CpTimetableResponse,
    };

    fn cp_station(code: &str, designation: &str) -> CpStation {
        CpStation {
            code: code.to_string(),
            designation: designation.to_string(),
            latitude: None,
            longitude: None,
            region: None,
            railways: None,
        }
    }

    fn search(query: &str) -> Vec<String> {
        let stations = [
            cp_station("94-69005", "Cais do Sodre"),
            cp_station("94-1008", "Porto Sao Bento"),
            cp_station("94-2006", "Porto Campanha"),
            cp_station("94-30007", "Lisboa Santa Apolonia"),
        ];
        CpAdapter::filter_stations(&stations, query)
            .into_iter()
            .map(|s| s.designation)
            .collect()
    }

    fn mock_adapter(base_url: &str) -> CpAdapter {
        CpAdapter::with_base_url(base_url, "key".into(), "id".into(), "secret".into())
    }

    #[tokio::test]
    async fn list_stations_returns_every_station() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/services/travel-api/stations"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"[{"code":"94-31039","designation":"Lisboa Oriente"},
                    {"code":"94-69005","designation":"Cais do Sodre"}]"#,
            ))
            .mount(&server)
            .await;

        let stations = mock_adapter(&server.uri()).list_stations().await.unwrap();

        let codes: Vec<_> = stations.response.iter().map(|s| s.code.as_str()).collect();
        assert_eq!(codes, ["94-31039", "94-69005"]);
        assert_eq!(stations.response[0].designation, "Lisboa Oriente");
    }

    #[tokio::test]
    async fn search_rejects_blank_query() {
        let err = mock_adapter("http://unused.invalid")
            .search_stations("   ")
            .await
            .unwrap_err();
        assert!(matches!(err, crate::error::CoreError::InvalidInput(_)));
    }

    #[test]
    fn search_ignores_accents_and_case() {
        assert_eq!(search("sodré"), ["Cais do Sodre"]);
        assert_eq!(search("CAMPANHÃ"), ["Porto Campanha"]);
        assert_eq!(search("Apolónia"), ["Lisboa Santa Apolonia"]);
    }

    #[test]
    fn search_matches_every_word_in_any_order() {
        assert_eq!(search("São Bento"), ["Porto Sao Bento"]);
        assert_eq!(search("bento  porto"), ["Porto Sao Bento"]);
        assert_eq!(search("porto"), ["Porto Sao Bento", "Porto Campanha"]);
        assert!(search("porto sodre").is_empty());
    }

    fn make_station(code: &str, designation: &str) -> CpStationSimple {
        CpStationSimple {
            code: code.to_string(),
            designation: designation.to_string(),
        }
    }

    fn make_stop() -> CpStationStop {
        CpStationStop {
            train_number: 120,
            train_service: CpServiceCode {
                code: "IC".to_string(),
                designation: "Intercidades".to_string(),
            },
            train_origin: make_station("94-001", "Lisboa"),
            train_destination: make_station("94-002", "Porto"),
            arrival_time: Some("10:00".to_string()),
            departure_time: Some("10:02".to_string()),
            platform: Some("3".to_string()),
            delay: Some(5),
            occupancy: None,
            eta: Some("10:05".to_string()),
            etd: Some("10:07".to_string()),
            supression: Some("Supressão".to_string()),
        }
    }

    #[test]
    fn convert_stop_maps_all_fields() {
        let stop = make_stop();
        let timetable = CpAdapter::convert_stop_to_timetable(&stop, "94-123");

        assert_eq!(timetable.train_number, 120);
        assert_eq!(timetable.service_type, "IC|Intercidades");
        assert_eq!(timetable.origin_station_name, "Lisboa");
        assert_eq!(timetable.destination_station_name, "Porto");
        assert_eq!(timetable.arrival_time, Some("10:00".to_string()));
        assert_eq!(timetable.departure_time, Some("10:02".to_string()));
        assert_eq!(timetable.estimated_arrival, Some("10:05".to_string()));
        assert_eq!(timetable.estimated_departure, Some("10:07".to_string()));
        assert_eq!(timetable.platform, Some("3".to_string()));
        assert_eq!(timetable.delay, Some(5));
        assert_eq!(timetable.observations, Some("Supressão".to_string()));
        assert_eq!(timetable.operator, "CP");
        assert!(!timetable.has_passed);
        assert!(timetable.is_departure);
    }

    #[test]
    fn convert_board_includes_all_stops_no_filtering() {
        let stop1 = make_stop();
        let mut stop2 = make_stop();
        stop2.eta = None;
        stop2.etd = None;
        let mut stop3 = make_stop();
        stop3.eta = None;

        let response = CpTimetableResponse {
            station_stops: vec![stop1, stop2, stop3],
            messages: vec![],
        };

        let board = CpAdapter::convert_timetable_to_board("94-123", &response);

        assert_eq!(board.trains.len(), 3);
        assert_eq!(board.station_id, "94-123");
        assert_eq!(board.station_name, "");
    }
}
