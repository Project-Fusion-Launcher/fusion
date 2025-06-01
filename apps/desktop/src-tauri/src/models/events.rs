use crate::models::{
    download::Download,
    game::{Game, GameSource},
};
use serde::Serialize;
use specta::Type;
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
    pub download_size: u64,
}
