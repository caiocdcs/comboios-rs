use std::sync::Arc;
use tokio::sync::RwLock;

use crate::adapters::{CpAdapter, CpConfigProvider, IpAdapter};
use crate::domain::{
    journey::TrainJourney, station::StationResponse, station_timetable::StationBoardResponse,
};
use crate::error::CoreError;

/// Async client for the CP (Comboios de Portugal) and IP (Infraestruturas de Portugal) APIs.
///
/// Holds shared, internally-locked adapters so it is cheap to clone and safe to share
/// across threads. All clones point to the same underlying connections and credentials.
///
/// # Examples
///
/// ```no_run
/// use comboios_core::Comboios;
///
/// #[tokio::main]
/// async fn main() -> Result<(), comboios_core::Error> {
///     let client = Comboios::new().await?;
///     let stations = client.search_stations("Porto").await?;
///     println!("Found {} stations", stations.response.len());
///     Ok(())
/// }
/// ```
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
    /// Create a new client, fetching live CP API credentials from `cp.pt`.
    ///
    /// On success the client is ready to use immediately. The credential fetch
    /// is cached internally; call [`refresh_credentials_from_website`] if you
    /// need to rotate them later.
    ///
    /// Also loads any `.env` file present in the working directory via `dotenvy`.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::NetworkError`] if `cp.pt` cannot be reached, or
    /// [`CoreError::ParseError`] if the credentials cannot be extracted from
    /// the response.
    ///
    /// [`refresh_credentials_from_website`]: Self::refresh_credentials_from_website
    pub async fn new() -> Result<Self, CoreError> {
        dotenvy::dotenv().ok();

        let config_provider = CpConfigProvider::new();

        let (api_key, connect_id, connect_secret) = config_provider.get_api_credentials().await?;

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

    /// Re-fetch CP API credentials from `cp.pt` and replace the ones currently
    /// in use.
    ///
    /// Useful when CP rotates its credentials and requests start returning
    /// [`CoreError::ApiError`] with a 401/403 status. All clones of this client
    /// share the same credentials, so a single call updates them everywhere.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::NetworkError`] if `cp.pt` cannot be reached, or
    /// [`CoreError::ParseError`] if the new credentials cannot be extracted.
    pub async fn refresh_credentials_from_website(&self) -> Result<(), CoreError> {
        tracing::info!("Refreshing CP credentials from cp.pt...");

        self.config_provider.invalidate_cache().await;

        let (api_key, connect_id, connect_secret) =
            self.config_provider.get_api_credentials().await?;

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

    /// Search for stations whose name contains `query` (case-insensitive).
    ///
    /// Returns all matching stations from the CP station index. An empty
    /// `query` string typically returns all known stations.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::NetworkError`] on connectivity failures or
    /// [`CoreError::ApiError`] if the CP API returns a non-success status.
    pub async fn search_stations(&self, query: &str) -> Result<StationResponse, CoreError> {
        let cp = self.cp.read().await;
        cp.search_stations(query).await
    }

    /// Retrieve the departure/arrival board for a station on a given date.
    ///
    /// - `station_id` — CP station identifier (e.g. `"94-31039"` for Lisboa-Oriente).
    /// - `date` — calendar date in `YYYY-MM-DD` format.
    /// - `start_time` — optional clock time in `HH:MM` format; when supplied only
    ///   trains departing or arriving at or after this time are returned.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::NetworkError`] on connectivity failures,
    /// [`CoreError::ApiError`] for non-success HTTP responses, or
    /// [`CoreError::InvalidInput`] if `station_id` or `date` are malformed.
    pub async fn get_station_timetable(
        &self,
        station_id: &str,
        date: &str,
        start_time: Option<&str>,
    ) -> Result<StationBoardResponse, CoreError> {
        let cp = self.cp.read().await;
        cp.get_station_timetable(station_id, date, start_time).await
    }

    /// Retrieve live journey details for a train, including stop-by-stop status
    /// and real-time delay information.
    ///
    /// The CP API is tried first. If it fails (e.g. the train is not yet in the
    /// CP system), the Infraestruturas de Portugal API is used as a fallback.
    /// Only the last error (from IP) is returned when both sources fail.
    ///
    /// - `train_number` — CP train identifier (e.g. `"120"`).
    /// - `date` — calendar date in `YYYY-MM-DD` format.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::NetworkError`] if neither source can be reached, or
    /// [`CoreError::ApiError`] / [`CoreError::ParseError`] if both adapters
    /// return errors.
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
                tracing::warn!(
                    "CP train journey failed for {}, trying IP: {}",
                    train_number,
                    e
                );
                match self.ip.get_train_journey(train_number, date).await {
                    Ok(Some(journey)) => {
                        tracing::info!("IP train journey succeeded for {}", train_number);
                        Ok(journey)
                    }
                    Ok(None) => {
                        tracing::warn!("IP train journey returned no data for {}", train_number);
                        Err(e)
                    }
                    Err(ip_err) => {
                        tracing::error!(
                            "Both CP and IP failed for train {}: CP={}, IP={}",
                            train_number,
                            e,
                            ip_err
                        );
                        Err(ip_err)
                    }
                }
            }
        }
    }

    /// Return a reference to the underlying config provider.
    ///
    /// Useful for inspecting cached credential state or integrating custom
    /// credential management logic.
    #[must_use]
    pub fn config_provider(&self) -> &CpConfigProvider {
        &self.config_provider
    }
}
