use crate::domain::{Station, StationBoard, StationBoardResponse, StationResponse};
use serde::Deserialize;

const BASE_URL: &str = "http://localhost:3000";

#[derive(Debug, Deserialize)]
struct StringResponse {
    data: String,
}

pub async fn search_station(name: &str) -> Result<Vec<Station>, reqwest::Error> {
    let url = format!("{}/stations?query={}", BASE_URL, name);
    let resp: StationResponse = reqwest::get(&url).await?.json().await?;
    Ok(resp.data)
}

pub async fn get_station_trains(station_id: &str) -> Result<Vec<StationBoard>, reqwest::Error> {
    let url = format!("{}/stations/timetable/{}", BASE_URL, station_id);
    let resp: StationBoardResponse = reqwest::get(&url).await?.json().await?;
    Ok(resp.data)
}

/// DEPRECATED: Train details API is no longer available
/// Returns a message explaining the deprecation
pub async fn get_train_details(train_id: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}/trains/{}", BASE_URL, train_id);
    let resp: StringResponse = reqwest::get(&url).await?.json().await?;
    Ok(resp.data)
}
