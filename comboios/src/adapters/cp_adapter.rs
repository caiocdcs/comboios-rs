use reqwest::Client;

use crate::domain::{
    journey::{JourneyStatus, JourneyStop, StopStatus, TrainJourney},
    station::StationResponse,
    station::Station as DomainStation,
    station_timetable::{StationBoard, StationBoardResponse, StationTimetable},
};
use crate::error::CoreError;
use crate::domain::cp_types::{CpStation, CpStationDetail, CpStationStop, CpTimetableResponse, CpTrainTimetable};

type Result<T> = std::result::Result<T, CoreError>;

const CP_STATIONS_API: &str = "https://api-gateway.cp.pt/cp/services/travel-api/stations";
const CP_STATION_INFO_API: &str = "https://api-gateway.cp.pt/cp/services/stations-api/stations/infos";
const CP_TRAINS_API: &str = "https://api-gateway.cp.pt/cp/services/travel-api/trains";

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36";

#[derive(Clone)]
pub struct CpAdapter {
    client: Client,
    api_key: String,
    connect_id: String,
    connect_secret: String,
}

impl CpAdapter {
    pub fn new(api_key: String, connect_id: String, connect_secret: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            connect_id,
            connect_secret,
        }
    }

    pub async fn search_stations(&self, query: &str) -> Result<StationResponse> {
        let url = CP_STATIONS_API;
        let stations: Vec<CpStation> = self.get(url).await?;
        
        let query_lower = query.to_lowercase();
        let matching: Vec<DomainStation> = stations
            .iter()
            .filter(|s| s.designation.to_lowercase().contains(&query_lower))
            .map(|s| DomainStation {
                code: s.code.clone(),
                designation: s.designation.clone(),
            })
            .collect();

        Ok(StationResponse { response: matching })
    }

    pub async fn get_station_detail(&self, station_id: &str) -> Result<CpStationDetail> {
        let url = format!("{}/{}", CP_STATION_INFO_API, station_id);
        self.get(&url).await
    }

    pub async fn get_station_timetable(
        &self,
        station_id: &str,
        date: &str,
        start_time: Option<&str>,
    ) -> Result<StationBoardResponse> {
        let mut url = format!("{}/{}/timetable/{}", CP_STATIONS_API, station_id, date);
        if let Some(start) = start_time {
            url.push_str(&format!("?start={}", start));
        }

        let response: CpTimetableResponse = self.get(&url).await?;

        let board = self.convert_timetable_to_board(station_id, &response, date);
        Ok(StationBoardResponse { response: vec![board] })
    }

    pub async fn get_train_journey(&self, train_number: &str, date: &str) -> Result<TrainJourney> {
        let url = format!("{}/{}/timetable/{}", CP_TRAINS_API, train_number, date);
        let timetable: CpTrainTimetable = self.get(&url).await?;
        Ok(self.convert_train_timetable(timetable))
    }

    fn convert_timetable_to_board(&self, station_id: &str, response: &CpTimetableResponse, _date: &str) -> StationBoard {
        let station_name = response
            .station_stops
            .iter()
            .find(|s| s.train_origin.code == station_id || s.train_destination.code == station_id)
            .map(|s| {
                if s.train_origin.code == station_id {
                    s.train_origin.designation.clone()
                } else {
                    s.train_destination.designation.clone()
                }
            })
            .unwrap_or_default();

        let trains: Vec<StationTimetable> = response
            .station_stops
            .iter()
            .map(|stop| self.convert_stop_to_timetable(stop, station_id))
            .collect();

        StationBoard {
            station_id: station_id.to_string(),
            station_name,
            trains,
        }
    }

    fn convert_stop_to_timetable(&self, stop: &CpStationStop, _station_id: &str) -> StationTimetable {
        // has_passed is true when actual times (ETD/ETA) are present, indicating train departed/arrived
        let has_passed = stop.etd.is_some() || stop.eta.is_some();
        let is_departure = stop.departure_time.is_some();

        StationTimetable {
            train_number: stop.train_number,
            service_type: format!("{}|{}", stop.train_service.code, stop.train_service.designation),
            origin_station_name: stop.train_origin.designation.clone(),
            origin_station_id: stop.train_origin.code.clone(),
            destination_station_name: stop.train_destination.designation.clone(),
            destination_station_id: stop.train_destination.code.clone(),
            departure_time: stop.departure_time.clone(),
            arrival_time: stop.arrival_time.clone(),
            platform: stop.platform.clone(),
            delay: stop.delay,
            operator: "CP".to_string(),
            has_passed,
            is_departure,
        }
    }

    fn convert_train_timetable(&self, timetable: CpTrainTimetable) -> TrainJourney {
        // Find the index of the last station the train has reached
        let last_passed_idx = timetable.last_station_code.as_ref().and_then(|code| {
            timetable.train_stops.iter().position(|s| &s.station.code == code)
        });

        // If train status is SCHEDULED, no stops have been passed yet
        let train_in_progress = timetable.status != "SCHEDULED";

        let stops: Vec<JourneyStop> = timetable
            .train_stops
            .iter()
            .enumerate()
            .map(|(i, stop)| {
                let scheduled_departure = stop.departure.clone().or(stop.arrival.clone());
                let scheduled_arrival = stop.arrival.clone().or(stop.departure.clone());

                let actual_departure = stop.etd.clone();
                let actual_arrival = stop.eta.clone();

                // Determine has_passed based on last_station_code or train status
                let has_passed = if let Some(last_idx) = last_passed_idx {
                    i <= last_idx
                } else if timetable.status == "PASSED" || timetable.status == "ARRIVED" {
                    true // Train completed, all stops passed
                } else {
                    false // Train not started or no last_station_code
                };

                let status = if !train_in_progress && i == 0 {
                    StopStatus::Scheduled
                } else if has_passed {
                    if i == 0 {
                        StopStatus::Departed
                    } else if Some(i) == last_passed_idx && actual_arrival.is_some() && actual_departure.is_none() {
                        StopStatus::AtStop
                    } else {
                        StopStatus::Passed
                    }
                } else {
                    StopStatus::Scheduled
                };

                JourneyStop {
                    station: DomainStation {
                        code: stop.station.code.clone(),
                        designation: stop.station.designation.clone(),
                    },
                    scheduled_arrival: scheduled_arrival.unwrap_or_default(),
                    scheduled_departure: scheduled_departure.unwrap_or_default(),
                    actual_arrival,
                    actual_departure,
                    platform: stop.platform.clone(),
                    status,
                    delay_minutes: stop.delay,
                    stop_number: i + 1,
                    has_passed: Some(has_passed),
                    predicted_time: if stop.eta.is_some() && stop.delay.unwrap_or(0) > 0 {
                        stop.eta.clone()
                    } else {
                        None
                    },
                }
            })
            .collect();

        let origin = timetable.train_stops.first().map(|s| DomainStation {
            code: s.station.code.clone(),
            designation: s.station.designation.clone(),
        }).unwrap_or(DomainStation {
            code: String::new(),
            designation: String::new(),
        });

        let destination = timetable.train_stops.last().map(|s| DomainStation {
            code: s.station.code.clone(),
            designation: s.station.designation.clone(),
        }).unwrap_or(DomainStation {
            code: String::new(),
            designation: String::new(),
        });

        let journey_status = if timetable.status == "PASSED" || timetable.status == "ARRIVED" {
            JourneyStatus::Completed
        } else if timetable.status == "NEAR_NEXT" || timetable.status == "AT_STATION" {
            JourneyStatus::InProgress
        } else if last_passed_idx.is_some() {
            // If last_station_code is set, train is in progress even if status says SCHEDULED
            JourneyStatus::InProgress
        } else {
            JourneyStatus::Scheduled
        };

        TrainJourney {
            train_number: timetable.train_number.to_string(),
            service_type: format!("{}|{}", timetable.service_code.code, timetable.service_code.designation),
            origin,
            destination,
            stops,
            status: journey_status,
            delay_minutes: timetable.delay,
            operator: "CP".to_string(),
            observations: None,
            duration: timetable.duration,
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
