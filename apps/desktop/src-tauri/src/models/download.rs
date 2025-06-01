use crate::models::game::GameSource;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug)]
pub struct Download {
    pub game_id: String,
    pub game_source: GameSource,
    pub game_version_id: String,
    pub game_title: String,
    pub path: PathBuf,
    pub download_size: u64,
    pub install_size: u64,
    pub downloaded: AtomicU64,
    pub written: AtomicU64,
}

impl Download {
    pub fn downloaded(&self) -> u64 {
        self.downloaded.load(Ordering::Relaxed)
    }

    pub fn set_downloaded(&self, value: u64) {
        self.downloaded.store(value, Ordering::Relaxed);
    }

    pub fn add_downloaded(&self, value: u64) {
        self.downloaded.fetch_add(value, Ordering::Relaxed);
    }

    pub fn written(&self) -> u64 {
        self.written.load(Ordering::Relaxed)
    }

    pub fn set_written(&self, value: u64) {
        self.written.store(value, Ordering::Relaxed);
    }

    pub fn add_written(&self, value: u64) {
        self.written.fetch_add(value, Ordering::Relaxed);
    }
}
