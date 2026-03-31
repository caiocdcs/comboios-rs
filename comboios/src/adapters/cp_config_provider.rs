use std::sync::Arc;
use std::time::{Duration, Instant};

use reqwest::Client;
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::error::CoreError;

const CP_CONFIG_URL: &str = "https://www.cp.pt/fe-config.json";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36";
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
        self.loaded_at.elapsed() > CONFIG_CACHE_TTL
    }
}

impl CpConfigProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config_cache: Arc::new(RwLock::new(ConfigCache {
                config: None,
                loaded_at: Instant::now() - CONFIG_CACHE_TTL,
            })),
        }
    }

    pub async fn invalidate_cache(&self) {
        let mut cache = self.config_cache.write().await;
        cache.config = None;
        cache.loaded_at = Instant::now() - CONFIG_CACHE_TTL;
    }

    pub async fn get_config(&self) -> Result<CpWebsiteConfig, CoreError> {
        let mut cache = self.config_cache.write().await;

        if !cache.is_expired() && cache.config.is_some() {
            return Ok(cache.config.clone().unwrap());
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
        Ok((
            config.travel_api_key,
            config.xcck,
            config.xccs,
        ))
    }
}

impl Default for CpConfigProvider {
    fn default() -> Self {
        Self::new()
    }
}
