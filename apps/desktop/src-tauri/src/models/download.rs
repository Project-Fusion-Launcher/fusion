use super::game::GameSource;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Download {
    pub game_id: String,
    pub game_source: GameSource,
    pub game_version_id: String,
    pub path: PathBuf,
    pub download_size: u64,
    pub install_size: u64,
    pub downloaded: u64,
    pub written: u64,
}

pub struct DownloadProgress {
    pub downloaded: u64,
    pub written: u64,
}
