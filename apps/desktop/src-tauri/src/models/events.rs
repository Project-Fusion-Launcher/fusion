use crate::models::{
    download::Download,
    game::{Game, GameSource},
};
use serde::Serialize;
use specta::Type;
use std::sync::Arc;
use tauri_specta::Event;

#[derive(Serialize, Debug, Type, Event, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameHidden {
    pub game_id: String,
    pub game_source: GameSource,
}

impl From<&Game> for GameHidden {
    fn from(game: &Game) -> Self {
        Self {
            game_id: game.id.clone(),
            game_source: game.source,
        }
    }
}

#[derive(Serialize, Debug, Type, Event, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameUninstalling {
    pub game_id: String,
    pub game_source: GameSource,
}

impl From<&Game> for GameUninstalling {
    fn from(game: &Game) -> Self {
        Self {
            game_id: game.id.clone(),
            game_source: game.source,
        }
    }
}

#[derive(Serialize, Debug, Type, Event, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameUninstalled {
    pub game_id: String,
    pub game_source: GameSource,
}

impl From<&Game> for GameUninstalled {
    fn from(game: &Game) -> Self {
        Self {
            game_id: game.id.clone(),
            game_source: game.source,
        }
    }
}

#[derive(Serialize, Debug, Type, Event, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameDownloadQueued {
    pub game_id: String,
    pub game_source: GameSource,
    pub game_title: String,
    pub download_size: u64,
    pub downloaded: u64,
}

impl From<&Download> for GameDownloadQueued {
    fn from(download: &Download) -> Self {
        Self {
            game_id: download.game_id.clone(),
            game_source: download.game_source,
            game_title: download.game_title.clone(),
            download_size: download.download_size,
            downloaded: download
                .downloaded
                .load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}

#[derive(Serialize, Debug, Type, Event, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameDownloadProgress {
    pub game_id: String,
    pub game_source: GameSource,
    pub downloaded: u64,
}

impl From<&Arc<Download>> for GameDownloadProgress {
    fn from(download: &Arc<Download>) -> Self {
        Self {
            game_id: download.game_id.clone(),
            game_source: download.game_source,
            downloaded: download
                .downloaded
                .load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}

#[derive(Serialize, Debug, Type, Event, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameDownloadFinished {
    pub game_id: String,
    pub game_source: GameSource,
}

impl From<&Arc<Download>> for GameDownloadFinished {
    fn from(download: &Arc<Download>) -> Self {
        Self {
            game_id: download.game_id.clone(),
            game_source: download.game_source,
        }
    }
}

#[derive(Serialize, Debug, Type, Event, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameInstalled {
    pub game_id: String,
    pub game_source: GameSource,
}

impl From<&Arc<Download>> for GameInstalled {
    fn from(download: &Arc<Download>) -> Self {
        Self {
            game_id: download.game_id.clone(),
            game_source: download.game_source,
        }
    }
}
