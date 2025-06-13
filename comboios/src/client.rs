use std::time::Duration;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{
    domain::{station::StationResponse, station_timetable::Timetable, train::Train},
    error::CoreError,
};

const CP_BASE_URL: &str = "https://www.cp.pt/sites/spring";
const IP_BASE_URL: &str = "https://www.infraestruturasdeportugal.pt";

#[tracing::instrument(skip(client))]
pub(crate) async fn get_request<T>(
    client: Client,
    url: String,
    timeout: Duration,
) -> Result<T, CoreError>
where
    T: DeserializeOwned,
{
    tracing::info!("GET request to url={:?} with timeout={:?}", url, timeout);
    println!("GET request to url={:?} with timeout={:?}", url, timeout);
    let response = client
        .get(url)
        .timeout(timeout)
        .header("User-Agent", "Chrome")
        .send()
        .await?;

    let data = response.json::<T>().await?;

    Ok(data)
}

pub async fn get_stations(
    client: Client,
    station_name: &str,
) -> Result<StationResponse, CoreError> {
    let timeout = Duration::from_secs(10);

    let url = format!(
        "{}/negocios-e-servicos/estacao-nome/{}",
        IP_BASE_URL, station_name
    );

    get_request(client, url, timeout).await
}

pub async fn get_station_timetable(
    client: Client,
    station_id: &str,
) -> Result<Vec<Timetable>, CoreError> {
    let timeout = Duration::from_secs(10);

    let formatted_station_id = format!("{}-{}", &station_id[..2], &station_id[2..]);

    let url = format!(
        "{}/station/trains?stationId={}",
        CP_BASE_URL, formatted_station_id
    );

    get_request(client, url, timeout).await
}

pub async fn get_train_details(client: Client, train_id: u16) -> Result<Train, CoreError> {
    let timeout = Duration::from_secs(10);

    let url = format!("{}/station/trains/train?trainId={}", CP_BASE_URL, train_id);

    get_request(client, url, timeout).await
}
