use chrono::Local;
use comboios_core::{Comboios, domain::station_timetable::StationBoard};
use rmcp::{
    Error as McpError, ServerHandler,
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    tool,
};

#[derive(Clone)]
pub struct CpServer {
    api: Comboios,
}

impl CpServer {
    pub async fn new() -> Self {
        match Comboios::new().await {
            Ok(api) => Self { api },
            Err(e) => {
                tracing::error!("Failed to initialize CP API: {e}");
                panic!("Failed to initialize CP API: {e}");
            }
        }
    }
}

#[tool(tool_box)]
impl CpServer {
    #[tool(description = "Get comboios stations by name")]
    async fn get_stations_by_name(
        &self,
        #[tool(param)]
        #[schemars(description = "Get comboios stations by name")]
        station_name: String,
    ) -> Result<CallToolResult, McpError> {
        match self.api.search_stations(&station_name).await {
            Ok(response) => {
                let stations = serde_json::to_string(&response)
                    .unwrap_or_else(|e| format!("Serialization error: {e}"));
                Ok(CallToolResult::success(vec![Content::text(stations)]))
            }
            Err(e) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Error searching stations: {e}"
            ))])),
        }
    }

    #[tool(description = "Get station timetable (departures/arrivals) for the next 12 hours")]
    async fn get_station_timetable(
        &self,
        #[tool(param)]
        #[schemars(description = "Station ID (e.g., 94-31039 for Lisboa-Oriente)")]
        station_id: String,
    ) -> Result<CallToolResult, McpError> {
        match self.fetch_station_timetable(&station_id).await {
            Ok(boards) => {
                let text = serde_json::to_string(&boards)
                    .unwrap_or_else(|e| format!("Serialization error: {e}"));
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(e) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Error fetching timetable: {e}"
            ))])),
        }
    }

    async fn fetch_station_timetable(&self, station_id: &str) -> Result<Vec<StationBoard>, String> {
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let start_time = now.format("%H:%M").to_string();

        self.api
            .get_station_timetable(station_id, &date, Some(&start_time))
            .await
            .map(|r| r.response)
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Get train journey details by train number")]
    async fn get_train_details(
        &self,
        #[tool(param)]
        #[schemars(description = "Train number (e.g., 18298)")]
        train_id: String,
    ) -> Result<CallToolResult, McpError> {
        let date = Local::now().format("%Y-%m-%d").to_string();
        match self.api.get_train_journey(&train_id, &date).await {
            Ok(journey) => {
                let message = serde_json::to_string(&journey)
                    .unwrap_or_else(|e| format!("Serialization error: {e}"));
                Ok(CallToolResult::success(vec![Content::text(message)]))
            }
            Err(e) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Error fetching train journey: {e}"
            ))])),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for CpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server provides tools to retrieve station information and timetables from Comboios de Portugal \
                 using the CP API with IP as fallback. Use get_station_timetable with a station_id to get current \
                 departures and arrivals."
                    .to_string(),
            ),
        }
    }
}
