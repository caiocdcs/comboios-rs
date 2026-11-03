use comboios_core::{Comboios, domain::station_timetable::StationBoard};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock, ServerCapabilities, ServerConfig},
    schemars::{self, JsonSchema},
    tool, tool_handler, tool_router,
};
use serde::Deserialize;

#[derive(Clone)]
pub struct CpServer {
    api: Comboios,
}

#[derive(Deserialize, JsonSchema)]
pub struct StationNameParams {
    #[schemars(description = "Get comboios stations by name")]
    station_name: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct StationIdParams {
    #[schemars(description = "Station ID (e.g., 94-31039 for Lisboa-Oriente)")]
    station_id: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct TrainIdParams {
    #[schemars(description = "Train number (e.g., 18298)")]
    train_id: String,
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

    async fn fetch_station_timetable(&self, station_id: &str) -> Result<Vec<StationBoard>, String> {
        let now = comboios_core::lisbon_now();
        let date = now.format("%Y-%m-%d").to_string();
        let start_time = now.format("%H:%M").to_string();

        self.api
            .get_station_timetable(station_id, &date, Some(&start_time))
            .await
            .map(|r| r.response)
            .map_err(|e| e.to_string())
    }
}

#[tool_router]
impl CpServer {
    #[tool(description = "Get comboios stations by name")]
    async fn get_stations_by_name(
        &self,
        Parameters(StationNameParams { station_name }): Parameters<StationNameParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.api.search_stations(&station_name).await {
            Ok(response) => {
                let stations = serde_json::to_string(&response)
                    .unwrap_or_else(|e| format!("Serialization error: {e}"));
                Ok(CallToolResult::success(vec![ContentBlock::text(stations)]))
            }
            Err(e) => Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "Error searching stations: {e}"
            ))])),
        }
    }

    #[tool(description = "Get station timetable (departures/arrivals) for the next 12 hours")]
    async fn get_station_timetable(
        &self,
        Parameters(StationIdParams { station_id }): Parameters<StationIdParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.fetch_station_timetable(&station_id).await {
            Ok(boards) => {
                let text = serde_json::to_string(&boards)
                    .unwrap_or_else(|e| format!("Serialization error: {e}"));
                Ok(CallToolResult::success(vec![ContentBlock::text(text)]))
            }
            Err(e) => Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "Error fetching timetable: {e}"
            ))])),
        }
    }

    #[tool(description = "Get train journey details by train number")]
    async fn get_train_details(
        &self,
        Parameters(TrainIdParams { train_id }): Parameters<TrainIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let date = comboios_core::lisbon_now().format("%Y-%m-%d").to_string();
        match self.api.get_train_journey(&train_id, &date).await {
            Ok(journey) => {
                let message = serde_json::to_string(&journey)
                    .unwrap_or_else(|e| format!("Serialization error: {e}"));
                Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
            }
            Err(e) => Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "Error fetching train journey: {e}"
            ))])),
        }
    }
}

#[tool_handler]
impl ServerHandler for CpServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            "This server provides tools to retrieve station information and timetables from Comboios de Portugal \
             using the CP API with IP as fallback. Use get_station_timetable with a station_id to get current \
             departures and arrivals.",
        )
    }
}
