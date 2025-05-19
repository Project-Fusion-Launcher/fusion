use super::game::GameSource;
use reqwest::RequestBuilder;
use std::path::PathBuf;

pub struct Download {
    pub files: Vec<DownloadFile>,
    pub chunks: Vec<DownloadChunk>,
    pub path: PathBuf,
    pub game_id: String,
    pub game_source: GameSource,
    pub game_title: String,
}

pub struct DownloadFile {
    pub filename: String,
    pub chunk_part: Vec<ChunkPart>,
    pub hash: DownloadHash,
}

pub struct ChunkPart {
    pub chunk_id: u128,
    pub size: u32,
    pub chunk_offset: u64,
    pub file_offset: u64,
}

pub struct DownloadChunk {
    pub id: u128,
    pub status: DownloadStatus,
    pub request: RequestBuilder,
    pub compressed_size: u64,
    pub size: u64,
    pub hash: DownloadHash,
}

#[derive(Clone)]
pub enum DownloadHash {
    Sha1(String),
    Sha256(String),
    Sha512(String),
    Md5(String),
    None,
}

#[derive(PartialEq)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Completed,
}
