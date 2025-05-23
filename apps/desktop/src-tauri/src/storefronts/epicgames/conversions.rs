use super::api::models::{Asset, Game as EpicGamesGame, GameManifestElement, KeyImageType};
use crate::models::game::{Game, GameSource, GameStatus, GameVersion};

impl From<EpicGamesGame> for Game {
    fn from(game: EpicGamesGame) -> Self {
        Game {
            id: game.id,
            sort_title: game.title.to_lowercase(),
            title: game.title,
            source: GameSource::EpicGames,
            key: None,
            developer: Some(game.developer),
            launch_target: None,
            path: None,
            version: None,
            status: GameStatus::NotInstalled,
            favorite: false,
            hidden: false,
            cover_url: game
                .key_images
                .iter()
                .find(|image| image.image_type == KeyImageType::DieselGameBoxTall)
                .map(|image| image.url.clone()),
        }
    }
}

impl From<GameManifestElement> for GameVersion {
    fn from(element: GameManifestElement) -> Self {
        GameVersion {
            id: element.build_version.clone(),
            name: element.build_version,
            external: false,
        }
    }
}

impl From<Asset> for GameVersion {
    fn from(asset: Asset) -> Self {
        GameVersion {
            id: asset.build_version.clone(),
            name: asset.build_version,
            external: false,
        }
    }
}
