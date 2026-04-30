use std::sync::Arc;
use std::time::{Duration, Instant};

use reqwest::Client;
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::constants::{CP_CONFIG_URL, USER_AGENT};
use crate::error::CoreError;

const CONFIG_CACHE_TTL: Duration = Duration::from_secs(60 * 60);

#[derive(Debug, Clone, Deserialize)]
pub struct CpWebsiteConfig {
    #[serde(rename = "travelApiUrl")]
    pub travel_api_url: String,
    #[serde(rename = "travelApiKey")]
    pub travel_api_key: String,
    #[serde(rename = "xcck")]
    pub xcck: String,
    #[serde(rename = "xccs")]
    pub xccs: String,
}

#[derive(Clone)]
pub struct CpConfigProvider {
    client: Client,
    config_cache: Arc<RwLock<ConfigCache>>,
}

struct ConfigCache {
    config: Option<CpWebsiteConfig>,
    loaded_at: Instant,
}

impl ConfigCache {
    fn is_expired(&self) -> bool {
        self.config.is_none() || self.loaded_at.elapsed() > CONFIG_CACHE_TTL
    }
}

impl CpConfigProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config_cache: Arc::new(RwLock::new(ConfigCache {
                config: None,
                loaded_at: Instant::now(),
            })),
        }
    }

    pub async fn invalidate_cache(&self) {
        let mut cache = self.config_cache.write().await;
        cache.config = None;
    }

    pub async fn get_config(&self) -> Result<CpWebsiteConfig, CoreError> {
        // Fast path: check under read lock first
        {
            let cache = self.config_cache.read().await;
            if !cache.is_expired()
                && let Some(ref config) = cache.config
            {
                return Ok(config.clone());
            }
        }

        // Slow path: refresh under write lock
        let mut cache = self.config_cache.write().await;
        // Re-check after acquiring write lock (another task may have refreshed)
        if !cache.is_expired()
            && let Some(ref config) = cache.config
        {
            return Ok(config.clone());
        }

        let config: CpWebsiteConfig = self.fetch_config().await?;
        cache.config = Some(config.clone());
        cache.loaded_at = Instant::now();

        Ok(config)
    }

    async fn fetch_config(&self) -> Result<CpWebsiteConfig, CoreError> {
        let response = self
            .client
            .get(CP_CONFIG_URL)
            .timeout(Duration::from_secs(10))
            .header("User-Agent", USER_AGENT)
            .header("Accept", "application/json")
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

        let config = response.json::<CpWebsiteConfig>().await?;
        Ok(config)
    }

    pub async fn get_travel_api_base_url(&self) -> Result<String, CoreError> {
        let config = self.get_config().await?;
        Ok(config.travel_api_url.clone())
    }

    pub async fn get_api_credentials(&self) -> Result<(String, String, String), CoreError> {
        let config = self.get_config().await?;
        Ok((config.travel_api_key, config.xcck, config.xccs))
    }
}

impl Default for CpConfigProvider {
    fn default() -> Self {
        Self::new()
    }
}
