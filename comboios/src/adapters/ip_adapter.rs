use std::time::Duration;

use reqwest::Client;

use crate::domain::{
    journey::TrainJourney,
    station::StationResponse,
    station_timetable::StationBoardResponse,
    train_journey::IpTrainJourneyWrapper,
};
use crate::error::CoreError;

const IP_BASE_URL: &str = "https://www.infraestruturasdeportugal.pt";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 Chrome/144.0.0.0 Safari/537.36";

#[derive(Clone)]
pub struct IpAdapter {
    client: Client,
    base_url: String,
}

impl IpAdapter {
    pub fn new() -> Self {
        Self::with_url(IP_BASE_URL)
    }

    pub fn with_url(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn search_stations(&self, query: &str) -> Result<StationResponse, CoreError> {
        let url = format!("{}/negocios-e-servicos/estacao-nome/{}", self.base_url, query);
        self.get(url).await
    }

    pub async fn get_station_timetable(
        &self,
        station_id: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<StationBoardResponse, CoreError> {
        let services = "INTERNACIONAL, ALFA, IC, IR, REGIONAL, URB|SUBUR, ESPECIAL";
        let url = format!(
            "{}/negocios-e-servicos/partidas-chegadas/{}/{}/{}/{}",
            self.base_url,
            station_id,
            urlencoding::encode(start_time),
            urlencoding::encode(end_time),
            urlencoding::encode(services)
        );
        self.get(url).await
    }

    pub async fn get_train_journey(&self, train_number: &str, date: &str) -> Result<TrainJourney, CoreError> {
        let url = format!(
            "{}/negocios-e-servicos/horarios-ncombio/{}/{}",
            self.base_url,
            train_number,
            urlencoding::encode(date)
        );

        let wrapper: IpTrainJourneyWrapper = self.get(url).await?;
        Ok(wrapper.response.to_train_journey(train_number))
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, url: String) -> Result<T, CoreError> {
        let response = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(30))
            .header("User-Agent", USER_AGENT)
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

impl Default for IpAdapter {
    fn default() -> Self {
        Self::new()
    }
}
