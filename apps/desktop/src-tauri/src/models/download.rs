use std::path::PathBuf;

use super::game::GameSource;

pub struct Download {
    pub files: Vec<DownloadFile>,
    pub path: PathBuf,
    pub game_id: String,
    pub game_source: GameSource,
    pub game_title: String,
}

impl Download {
    pub fn download_size(&self) -> u64 {
        self.files
            .iter()
            .flat_map(|file| &file.chunks)
            .map(|chunk| chunk.size as u64)
            .sum()
    }
}

pub struct DownloadFile {
    pub filename: String,
    pub chunks: Vec<DownloadFileChunk>,
    pub hash: DownloadHash,
}

pub struct DownloadFileChunk {
    pub hash: DownloadHash,
    pub size: u32,
    pub offset: u64,
}

pub enum DownloadHash {
    Sha1(String),
    Sha256(String),
    Sha512(String),
    Md5(String),
    None,
}
