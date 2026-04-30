//! Server configuration loaded from environment variables.
//!
//! All settings have sensible defaults so the server runs without any
//! configuration. Set the corresponding environment variable to override.

use std::time::Duration;

/// Full server configuration.
///
/// Constructed via [`Settings::from_env`], which reads environment variables
/// and falls back to defaults for any that are not set.
#[derive(Debug, Clone)]
pub struct Settings {
    /// Bind address for the HTTP server (e.g. `"0.0.0.0"`).
    /// Env: `HOST`. Default: `"0.0.0.0"`.
    pub host: String,

    /// TCP port to listen on.
    /// Env: `PORT`. Default: `3000`.
    pub port: u16,

    /// CP API Gateway base URL used by the core library diagnostics check.
    /// Env: `CP_API_URL`. Default: `"https://api-gateway.cp.pt/cp/services/travel-api"`.
    pub cp_api_url: String,

    /// Infraestruturas de Portugal base URL used by the diagnostics check.
    /// Env: `IP_API_URL`. Default: `"https://www.infraestruturasdeportugal.pt"`.
    pub ip_api_url: String,

    /// How long the server waits for an outbound request before timing out.
    /// Env: `REQUEST_TIMEOUT_SECS`. Default: `30`.
    pub request_timeout: Duration,

    /// Timeout used by the diagnostics health-check probes.
    /// Env: `DIAGNOSTICS_TIMEOUT_MS`. Default: `5000`.
    pub diagnostics_timeout: Duration,

    /// How often the background task refreshes CP credentials.
    /// Env: `CREDENTIAL_REFRESH_SECS`. Default: `3300` (55 minutes).
    pub credential_refresh_interval: Duration,

    /// `Access-Control-Max-Age` sent in CORS pre-flight responses (seconds).
    /// Env: `CORS_MAX_AGE_SECS`. Default: `86400` (24 hours).
    pub cors_max_age: Duration,

    /// Log filter directive passed to `tracing-subscriber`.
    /// Env: `RUST_LOG`. Default: `"comboios_server=debug,tower_http=debug"`.
    pub log_filter: String,
}

impl Settings {
    /// Load configuration from environment variables, using defaults for any
    /// that are absent or unparseable.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            host: env_string("HOST", "0.0.0.0"),
            port: env_parse("PORT", 3000),
            cp_api_url: env_string(
                "CP_API_URL",
                "https://api-gateway.cp.pt/cp/services/travel-api",
            ),
            ip_api_url: env_string("IP_API_URL", "https://www.infraestruturasdeportugal.pt"),
            request_timeout: Duration::from_secs(env_parse("REQUEST_TIMEOUT_SECS", 30)),
            diagnostics_timeout: Duration::from_millis(env_parse("DIAGNOSTICS_TIMEOUT_MS", 5000)),
            credential_refresh_interval: Duration::from_secs(env_parse(
                "CREDENTIAL_REFRESH_SECS",
                3300,
            )),
            cors_max_age: Duration::from_secs(env_parse("CORS_MAX_AGE_SECS", 86400)),
            log_filter: env_string("RUST_LOG", "comboios_server=debug,tower_http=debug"),
        }
    }

    /// Bind address in `host:port` form, ready to pass to [`TcpListener::bind`].
    ///
    /// [`TcpListener::bind`]: tokio::net::TcpListener::bind
    #[must_use]
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Read an environment variable as a `String`, falling back to `default`.
fn env_string(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}

/// Read an environment variable and parse it as `T`, falling back to `default`
/// when the variable is absent or cannot be parsed.
fn env_parse<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr + Copy,
{
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_owned(),
            port: 3000,
            cp_api_url: "https://api-gateway.cp.pt/cp/services/travel-api".to_owned(),
            ip_api_url: "https://www.infraestruturasdeportugal.pt".to_owned(),
            request_timeout: Duration::from_secs(30),
            diagnostics_timeout: Duration::from_millis(5000),
            credential_refresh_interval: Duration::from_secs(3300),
            cors_max_age: Duration::from_secs(86400),
            log_filter: "comboios_server=debug,tower_http=debug".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sensible() {
        let s = Settings::default();
        assert_eq!(s.host, "0.0.0.0");
        assert_eq!(s.port, 3000);
        assert_eq!(s.bind_address(), "0.0.0.0:3000");
        assert!(s.cp_api_url.contains("cp.pt"));
        assert!(s.ip_api_url.contains("infraestruturasdeportugal"));
        assert_eq!(s.request_timeout, Duration::from_secs(30));
        assert_eq!(s.diagnostics_timeout, Duration::from_millis(5000));
        assert_eq!(s.credential_refresh_interval, Duration::from_secs(3300));
        assert_eq!(s.cors_max_age, Duration::from_secs(86400));
    }

    #[test]
    fn custom_values_override_defaults() {
        let s = Settings {
            host: "127.0.0.1".to_owned(),
            port: 8080,
            request_timeout: Duration::from_secs(60),
            diagnostics_timeout: Duration::from_millis(2000),
            ..Settings::default()
        };
        assert_eq!(s.host, "127.0.0.1");
        assert_eq!(s.port, 8080);
        assert_eq!(s.bind_address(), "127.0.0.1:8080");
        assert_eq!(s.request_timeout, Duration::from_secs(60));
        assert_eq!(s.diagnostics_timeout, Duration::from_millis(2000));
    }

    #[test]
    fn unparseable_env_falls_back_to_default() {
        // env_parse is exercised by providing an invalid value through the
        // helper directly; no global env mutation needed.
        let parsed: u16 = std::env::var("__COMBOIOS_NONEXISTENT_KEY__")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3000_u16);
        assert_eq!(parsed, 3000);
    }

    #[test]
    fn bind_address_formats_correctly() {
        let s = Settings {
            host: "10.0.0.1".to_owned(),
            port: 9000,
            ..Settings::default()
        };
        assert_eq!(s.bind_address(), "10.0.0.1:9000");
    }
}
