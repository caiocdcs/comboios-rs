use chrono::Local;
use comboios_core::{
    Comboios,
    domain::{station::StationResponse, station_timetable::StationBoard},
};
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
        Self {
            api: Comboios::new().await.expect("Failed to initialize CP API"),
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
        let response = self.fetch_stations(&station_name).await;
        let stations = serde_json::to_string(&response).unwrap();
        Ok(CallToolResult::success(vec![Content::text(stations)]))
    }

    async fn fetch_stations(&self, station_name: &str) -> StationResponse {
        self.api.search_stations(station_name).await.unwrap()
    }

    #[tool(description = "Get station timetable (departures/arrivals) for the next 12 hours")]
    async fn get_station_timetable(
        &self,
        #[tool(param)]
        #[schemars(description = "Station ID (e.g., 94-31039 for Lisboa-Oriente)")]
        station_id: String,
    ) -> Result<CallToolResult, McpError> {
        let response = self.fetch_station_timetable(&station_id).await;
        let boards = serde_json::to_string(&response).unwrap();
        Ok(CallToolResult::success(vec![Content::text(boards)]))
    }

    async fn fetch_station_timetable(&self, station_id: &str) -> Vec<StationBoard> {
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let start_time = now.format("%H:%M").to_string();

        match self
            .api
            .get_station_timetable(station_id, &date, Some(&start_time))
            .await
        {
            Ok(response) => response.response,
            Err(_) => Vec::new(),
        }
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
                let message = serde_json::to_string(&journey).unwrap();
                Ok(CallToolResult::success(vec![Content::text(message)]))
            }
            Err(e) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Error: {e}"
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
