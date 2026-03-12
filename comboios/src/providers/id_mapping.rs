//! Station ID mapping between IP and CP API formats
//!
//! IP API uses: "9431039" (numeric, no dashes)
//! CP API uses: "94-60103" (with dash after first 2 digits)

/// Convert IP station ID to CP format
///
/// # Examples
/// ```
/// use comboios::providers::to_cp_id;
///
/// assert_eq!(to_cp_id("9431039"), "94-31039");
/// assert_eq!(to_cp_id("9402006"), "94-02006");
/// ```
pub fn to_cp_id(ip_id: &str) -> String {
    if ip_id.len() < 2 {
        return ip_id.to_string();
    }

    // Split into first 2 digits and rest
    let prefix = &ip_id[..2];
    let suffix = &ip_id[2..];

    // CP format keeps leading zeros in suffix
    format!("{}-{}", prefix, suffix)
}

/// Convert CP station ID to IP format
///
/// # Examples
/// ```
/// use comboios::providers::to_ip_id;
///
/// assert_eq!(to_ip_id("94-31039"), "9431039");
/// assert_eq!(to_ip_id("94-20006"), "9420006");
/// ```
pub fn to_ip_id(cp_id: &str) -> String {
    cp_id.replace("-", "")
}

/// Normalize station ID to IP format (removes dashes)
pub fn normalize_station_id(id: &str) -> String {
    id.replace("-", "").trim().to_string()
}

/// Check if ID is in CP format (contains dash)
pub fn is_cp_format(id: &str) -> bool {
    id.contains('-')
}

/// Check if ID is in IP format (numeric only)
pub fn is_ip_format(id: &str) -> bool {
    !id.contains('-') && id.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_cp_id() {
        assert_eq!(to_cp_id("9431039"), "94-31039");
        assert_eq!(to_ip_id("94-31039"), "9431039");

        assert_eq!(to_cp_id("9402006"), "94-02006");
        assert_eq!(to_ip_id("94-02006"), "9402006");

        // Round-trip
        let original = "9431039";
        let cp = to_cp_id(original);
        let back = to_ip_id(&cp);
        assert_eq!(original, back);
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize_station_id("94-60103"), "9460103");
        assert_eq!(normalize_station_id("9431039"), "9431039");
    }

    #[test]
    fn test_format_detection() {
        assert!(is_cp_format("94-60103"));
        assert!(!is_cp_format("9431039"));

        assert!(is_ip_format("9431039"));
        assert!(!is_ip_format("94-60103"));
    }
}
