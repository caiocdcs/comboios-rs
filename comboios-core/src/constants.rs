//! Shared constants for HTTP adapters.

/// Browser user-agent sent with every outbound request.
///
/// Matches a recent Chrome release on macOS, which is what cp.pt and
/// infraestruturasdeportugal.pt expect to see from a normal browser session.
pub(crate) const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
     AppleWebKit/537.36 (KHTML, like Gecko) \
     Chrome/144.0.0.0 Safari/537.36";

/// Default base URL for the CP API Gateway.
///
/// Can be overridden at construction time via `CpAdapter::with_base_url` for
/// testing or when pointing at a proxy.
pub(crate) const CP_BASE_URL: &str = "https://api-gateway.cp.pt/cp";

/// Default URL for the CP website configuration JSON.
///
/// This endpoint returns the live API key and connect credentials that
/// `CpConfigProvider` caches and rotates automatically.
pub(crate) const CP_CONFIG_URL: &str = "https://www.cp.pt/fe-config.json";

/// Default base URL for the Infraestruturas de Portugal public API.
pub(crate) const IP_BASE_URL: &str = "https://www.infraestruturasdeportugal.pt";
