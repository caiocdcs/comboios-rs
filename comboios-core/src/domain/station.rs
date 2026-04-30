use serde::{Deserialize, Deserializer, Serialize};

fn int_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(s),
        Value::Number(n) => Ok(n.to_string()),
        _ => Err(Error::custom("expected string or number")),
    }
}

/// A CP station with its unique code and human-readable name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    /// CP station identifier (e.g. `"94-31039"` for Lisboa-Oriente).
    ///
    /// Accepts both string and integer representations from the API and
    /// normalises them to a `String`.
    #[serde(alias = "NodeID", deserialize_with = "int_to_string")]
    pub code: String,
    /// Human-readable station name (e.g. `"Lisboa - Oriente"`).
    #[serde(alias = "Nome")]
    pub designation: String,
}

/// Response wrapper returned by [`crate::Comboios::search_stations`].
///
/// The `response` field contains every station whose name matches the search
/// query, in the order returned by the CP API.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StationResponse {
    /// Matching stations; may be empty when no station name contains the query.
    pub response: Vec<Station>,
}
