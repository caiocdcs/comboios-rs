use comboios::ComboiosApi;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AppState {
    pub(crate) api: ComboiosApi,
}

#[derive(Debug, Serialize)]
pub struct AppResponse<T> {
    pub(crate) data: T,
}

#[derive(Debug, Deserialize)]
pub struct TrainId(u16);

impl Into<u16> for TrainId {
    fn into(self) -> u16 {
        self.0
    }
}
