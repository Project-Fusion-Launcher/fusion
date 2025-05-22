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

pub struct DownloadManifest {
    pub chunks: Vec<DownloadChunk>,
    pub files: Vec<DownloadFile>,
}

impl DownloadManifest {
    pub fn download_size(&self) -> u64 {
        self.chunks.iter().map(|chunk| chunk.compressed_size).sum()
    }

    pub fn chunk_files(&self, chunk_id: u128) -> Vec<DownloadFile> {
        let mut result = Vec::new();

        for file in &self.files {
            for part in &file.chunk_parts {
                if part.id == chunk_id {
                    result.push(DownloadFile {
                        filename: file.filename.clone(),
                        hash: file.hash.clone(),
                        chunk_parts: vec![part.clone()],
                    });
                }
            }
        }

        result
    }
}

#[derive(Debug)]
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

#[derive(Clone, Debug)]
pub struct DownloadChunkPart {
    pub id: u128,
    pub chunk_offset: u64,
    pub file_offset: u64,
    pub size: u64,
    pub completed: bool,
}

#[derive(Clone)]
pub struct DownloadChunk {
    pub id: u128,
    pub completed: bool,
    pub url: String,
    pub compressed_size: u64,
    pub size: u64,
    pub hash: DownloadHash,
}

#[derive(Clone, Debug)]
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

#[derive(Serialize, Deserialize)]
pub struct DownloadState {
    pub completed_chunks: Vec<u128>,
    //pub completed_files: Vec<>
}
