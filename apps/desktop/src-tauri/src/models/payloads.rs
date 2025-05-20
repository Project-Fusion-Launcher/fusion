use std::path::PathBuf;

use super::game::GameSource;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadOptions {
    pub install_location: PathBuf,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DownloadPayload {
    pub game_id: String,
    pub game_source: GameSource,
    pub game_title: String,
    pub download_size: u64,
    pub downloaded: u64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameFiltersPayload {
    pub query: Option<String>,
}
