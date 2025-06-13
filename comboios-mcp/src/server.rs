use comboios::{
    ComboiosApi,
    domain::{station::StationResponse, station_timetable::Timetable, train::Train},
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

    #[tool(description = "Get stations by name")]
    async fn get_stations_by_name(
        &self,
        #[tool(param)]
        #[schemars(description = "Get stations by name")]
        station_name: String,
    ) -> Result<CallToolResult, McpError> {
        let response = self._get_stations_from(&station_name).await;
        let stations = serde_json::to_string(&response).unwrap();
        Ok(CallToolResult::success(vec![Content::text(stations)]))
    }

    async fn _get_stations_from(&self, station_name: &str) -> StationResponse {
        self.api.get_stations(station_name).await.unwrap()
    }

    #[tool(description = "Get station timetable by station id")]
    async fn get_station_timetable(
        &self,
        #[tool(param)]
        #[schemars(description = "Get station timetable by station id")]
        station_id: String,
    ) -> Result<CallToolResult, McpError> {
        let response = self._get_station_timetable(&station_id).await;
        let stations = serde_json::to_string(&response).unwrap();
        Ok(CallToolResult::success(vec![Content::text(stations)]))
    }

    async fn _get_station_timetable(&self, station_id: &str) -> Vec<Timetable> {
        self.api.get_station_timetable(station_id).await.unwrap()
    }

    #[tool(description = "Get train details by train id")]
    async fn get_train_details(
        &self,
        #[tool(param)]
        #[schemars(description = "Get train details by train id")]
        train_id: u16,
    ) -> Result<CallToolResult, McpError> {
        let response = self._get_train_details(train_id).await;
        let stations = serde_json::to_string(&response).unwrap();
        Ok(CallToolResult::success(vec![Content::text(stations)]))
    }

    async fn _get_train_details(&self, train_id: u16) -> Train {
        self.api.get_train_details(train_id).await.unwrap()
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
            instructions: Some("This server provides tools to retrieve station and trains from the Comboios de Portugal".to_string()),
        }
    }
}
