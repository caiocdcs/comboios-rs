use comboios_core::Comboios;
use serde::{Deserialize, Serialize};

use crate::configuration::Settings;

#[derive(Debug)]
pub struct AppState {
    pub(crate) api: Comboios,
    pub(crate) settings: Settings,
}

#[derive(Debug, Serialize)]
pub struct AppResponse<T> {
    pub(crate) data: T,
}

#[derive(Debug, Deserialize)]
pub struct TrainId(u16);

impl From<TrainId> for u16 {
    fn from(val: TrainId) -> Self {
        val.0
    }
}
