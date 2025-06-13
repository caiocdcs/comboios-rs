use crate::domain::{
    Station, StationResponse, StopoverResponse, Timetable, TrainDetails, TrainDetailsResponse,
};

const BASE_URL: &str = "http://localhost:3000";

pub async fn search_station(name: &str) -> Result<Vec<Station>, reqwest::Error> {
    let url = format!("{}/stations?query={}", BASE_URL, name);
    let resp: StationResponse = reqwest::get(&url).await?.json().await?;
    Ok(resp.data)
}

pub async fn get_station_trains(station_id: &str) -> Result<Vec<Timetable>, reqwest::Error> {
    let url = format!("{}/stations/timetable/{}", BASE_URL, station_id);
    let resp: StopoverResponse = reqwest::get(&url).await?.json().await?;
    Ok(resp.data)
}

pub async fn get_train_details(train_id: &str) -> Result<TrainDetails, reqwest::Error> {
    let url = format!("{}/trains/{}", BASE_URL, train_id);
    let resp: TrainDetailsResponse = reqwest::get(&url).await?.json().await?;
    Ok(resp.data)
}
