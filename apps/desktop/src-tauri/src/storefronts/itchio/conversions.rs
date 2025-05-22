use super::api::models::{OwnedKey, Upload, UploadStorage};
use crate::models::game::{Game, GameSource, GameStatus, GameVersion};

impl From<OwnedKey> for Game {
    fn from(key: OwnedKey) -> Self {
        let developer = key
            .game
            .user
            .map(|user| user.display_name.unwrap_or(user.username));

        Game {
            id: key.game.id.to_string(),
            sort_title: key.game.title.to_lowercase(),
            title: key.game.title,
            source: GameSource::Itchio,
            key: Some(key.id.to_string()),
            developer,
            launch_target: None,
            path: None,
            version: None,
            status: GameStatus::NotInstalled,
            favorite: false,
            hidden: false,
            cover_url: key.game.still_cover_url.or(key.game.cover_url),
        }
    }
}

impl From<Upload> for GameVersion {
    fn from(upload: Upload) -> Self {
        GameVersion {
            id: upload.id.to_string(),
            name: upload.display_name.unwrap_or(upload.filename),
            external: upload.storage == UploadStorage::External,
        }
    }
}
