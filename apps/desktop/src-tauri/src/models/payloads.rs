use serde::Deserialize;
use specta::Type;
use std::path::PathBuf;

#[derive(Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DownloadOptions {
    pub install_location: PathBuf,
}

#[derive(Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct GameFilters {
    pub query: Option<String>,
}
