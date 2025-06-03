use super::api::models::{Asset, Game as EpicGamesGame, GameManifestElement, KeyImageType};
use crate::models::game::{Game, GameBuilder, GameSource, GameVersion};

impl From<EpicGamesGame> for Game {
    fn from(game: EpicGamesGame) -> Self {
        let cover_url = game
            .key_images
            .into_iter()
            .find(|image| image.image_type == KeyImageType::DieselGameBoxTall)
            .map(|image| image.url);

        let mut builder =
            GameBuilder::new(game.id, GameSource::EpicGames, game.title).developer(game.developer);

        if let Some(cover) = cover_url {
            builder = builder.cover_url(cover);
        }

        builder.build()
    }
}

impl From<GameManifestElement> for GameVersion {
    fn from(element: GameManifestElement) -> Self {
        Self {
            id: element.build_version.clone(),
            name: element.build_version,
            external: false,
        }
    }
}

impl From<Asset> for GameVersion {
    fn from(asset: Asset) -> Self {
        Self {
            id: asset.build_version.clone(),
            name: asset.build_version,
            external: false,
        }
    }
}
