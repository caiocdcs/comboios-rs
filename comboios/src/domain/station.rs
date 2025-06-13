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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    #[serde(alias = "NodeID", deserialize_with = "int_to_string")]
    pub code: String,
    #[serde(alias = "Nome")]
    pub designation: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StationResponse {
    pub response: Vec<Station>,
}
