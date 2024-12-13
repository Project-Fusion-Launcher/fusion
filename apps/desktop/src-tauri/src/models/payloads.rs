use super::game::GameSource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DownloadFinishedPayload {
    pub game_id: String,
    pub game_source: GameSource,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameUninstalledPayload {
    pub game_id: String,
    pub game_source: GameSource,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameFiltersPayload {
    pub query: Option<String>,
}
