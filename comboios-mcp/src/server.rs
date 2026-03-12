use comboios::{
    ComboiosApi,
    domain::{
        station::StationResponse,
        station_timetable::StationBoard,
    },
};
use rmcp::{Error as McpError, ServerHandler, model::*, tool};

#[derive(Clone)]
pub struct CpServer {
    api: ComboiosApi,
}

#[tool(tool_box)]
impl CpServer {
    pub fn new() -> Self {
        Self {
            api: ComboiosApi::new(),
        }
    }

    #[tool(description = "Get comboios stations by name")]
    async fn get_stations_by_name(
        &self,
        #[tool(param)]
        #[schemars(description = "Get comboios stations by name")]
        station_name: String,
    ) -> Result<CallToolResult, McpError> {
        let response = self._get_stations_from(&station_name).await;
        let stations = serde_json::to_string(&response).unwrap();
        Ok(CallToolResult::success(vec![Content::text(stations)]))
    }

    async fn _get_stations_from(&self, station_name: &str) -> StationResponse {
        self.api.get_stations(station_name).await.unwrap()
    }

    #[tool(description = "Get station timetable (departures/arrivals) for the next 12 hours")]
    async fn get_station_timetable(
        &self,
        #[tool(param)]
        #[schemars(description = "Station ID (e.g., 9431039 for Lisboa-Oriente)")]
        station_id: String,
    ) -> Result<CallToolResult, McpError> {
        let response = self._get_station_timetable(&station_id).await;
        let boards = serde_json::to_string(&response).unwrap();
        Ok(CallToolResult::success(vec![Content::text(boards)]))
    }

    async fn _get_station_timetable(&self, station_id: &str) -> Vec<StationBoard> {
        self.api
            .get_station_board_now(station_id)
            .await
            .unwrap_or_default()
    }

    #[tool(description = "Get train journey details by train number (deprecated - no longer available)")]
    async fn get_train_details(
        &self,
        #[tool(param)]
        #[schemars(description = "Train number (e.g., 722)")]
        train_id: u16,
    ) -> Result<CallToolResult, McpError> {
        let response = self._get_train_details(train_id).await;
        let message = serde_json::to_string(&response).unwrap();
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    async fn _get_train_details(&self, _train_id: u16) -> String {
        "Train details API is no longer available from Comboios de Portugal. \
         Please use get_station_timetable to see current departures and arrivals."
            .to_string()
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
                 using the Infraestruturas de Portugal API. Use get_station_timetable with a station_id to get current \
                 departures and arrivals for the next 12 hours."
                    .to_string(),
            ),
        }
    }
}
