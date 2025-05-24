use super::game::GameSource;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Download {
    pub game_id: String,
    pub game_source: GameSource,
    pub game_version_id: String,
    pub path: PathBuf,
    pub completed: bool,
}

pub struct DownloadProgress {
    pub chunk_id: u128,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadState {
    pub completed_chunks: Vec<u128>,
    //pub completed_files: Vec<>
}
