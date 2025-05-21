use super::game::GameSource;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Download {
    pub chunks: Vec<DownloadChunk>,
    pub files: Vec<DownloadFile>,
    pub path: PathBuf,
    pub game_id: String,
    pub game_source: GameSource,
    pub game_title: String,
}

impl Download {
    pub fn download_size(&self) -> u64 {
        self.chunks.iter().map(|chunk| chunk.compressed_size).sum()
    }
}

#[derive(Serialize, Deserialize)]
pub struct DownloadFile {
    pub filename: String,
    pub hash: DownloadHash,
    pub chunk_parts: Vec<DownloadChunkPart>,
}

impl DownloadFile {
    pub fn status(&self) -> bool {
        self.chunk_parts.iter().all(|part| part.completed)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DownloadChunkPart {
    pub id: u128,
    pub chunk_offset: u64,
    pub file_offset: u64,
    pub size: u64,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadChunk {
    pub id: u128,
    pub completed: bool,
    pub url: String,
    pub compressed_size: u64,
    pub size: u64,
    pub hash: DownloadHash,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DownloadHash {
    Sha1(String),
    Sha256(String),
    Sha512(String),
    Md5(String),
    None,
}

pub struct DownloadProgress {
    pub chunk_id: u128,
    pub completed: bool,
}
