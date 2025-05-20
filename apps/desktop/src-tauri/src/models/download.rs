use super::game::GameSource;
use reqwest::RequestBuilder;
use std::path::PathBuf;

pub struct Download {
    pub chunks: Vec<DownloadChunk>,
    pub path: PathBuf,
    pub game_id: String,
    pub game_source: GameSource,
    pub game_title: String,
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
