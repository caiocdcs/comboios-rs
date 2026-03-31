use std::sync::Arc;
use tokio::sync::RwLock;

use crate::adapters::{CpAdapter, CpConfigProvider, IpAdapter};
use crate::error::CoreError;
use crate::domain::{
    journey::TrainJourney,
    station::StationResponse,
    station_timetable::StationBoardResponse,
};

#[derive(Clone)]
pub struct Comboios {
    cp: Arc<RwLock<CpAdapter>>,
    ip: IpAdapter,
    config_provider: CpConfigProvider,
}

impl std::fmt::Debug for Comboios {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Comboios").finish()
    }
}

impl Comboios {
    pub async fn new() -> Result<Self, CoreError> {
        dotenvy::dotenv().ok();

        let config_provider = CpConfigProvider::new();
        
        let (api_key, connect_id, connect_secret) = config_provider
            .get_api_credentials()
            .await?;

        tracing::info!(
            "Loaded credentials from cp.pt: API key starts with {}, ID starts with {}",
            &api_key[..8.min(api_key.len())],
            &connect_id[..8.min(connect_id.len())]
        );

        Ok(Self {
            cp: Arc::new(RwLock::new(CpAdapter::new(
                api_key,
                connect_id,
                connect_secret,
            ))),
            ip: IpAdapter::new(),
            config_provider,
        })
    }

    pub async fn refresh_credentials_from_website(&self) -> Result<(), CoreError> {
        tracing::info!("Refreshing CP credentials from cp.pt...");

        self.config_provider.invalidate_cache().await;

        let (api_key, connect_id, connect_secret) = self
            .config_provider
            .get_api_credentials()
            .await?;

        tracing::info!(
            "Refreshed credentials: API key starts with {}, ID starts with {}",
            &api_key[..8.min(api_key.len())],
            &connect_id[..8.min(connect_id.len())]
        );

        let new_cp = CpAdapter::new(api_key, connect_id, connect_secret);
        let mut cp = self.cp.write().await;
        *cp = new_cp;

        Ok(())
    }

    pub async fn search_stations(&self, query: &str) -> Result<StationResponse, CoreError> {
        let cp = self.cp.read().await;
        cp.search_stations(query).await
    }

    pub async fn get_station_timetable(
        &self,
        station_id: &str,
        date: &str,
        start_time: Option<&str>,
    ) -> Result<StationBoardResponse, CoreError> {
        let cp = self.cp.read().await;
        cp.get_station_timetable(station_id, date, start_time)
            .await
    }

    pub async fn get_train_journey(
        &self,
        train_number: &str,
        date: &str,
    ) -> Result<TrainJourney, CoreError> {
        let cp = self.cp.read().await;
        match cp.get_train_journey(train_number, date).await {
            Ok(journey) => {
                tracing::debug!("CP train journey succeeded for {}", train_number);
                Ok(journey)
            }
            Err(e) => {
                tracing::warn!("CP train journey failed for {}, trying IP: {}", train_number, e);
                match self.ip.get_train_journey(train_number, date).await {
                    Ok(journey) => {
                        tracing::info!("IP train journey succeeded for {}", train_number);
                        Ok(journey)
                    }
                    Err(ip_err) => {
                        tracing::error!("Both CP and IP failed for train {}: CP={}, IP={}", train_number, e, ip_err);
                        Err(ip_err)
                    }
                }
            }
        }
    }

    pub fn config_provider(&self) -> &CpConfigProvider {
        &self.config_provider
    }
}

impl Default for Comboios {
    fn default() -> Self {
        panic!("Comboios::new() must be used instead of Comboios::default()")
    }
}
