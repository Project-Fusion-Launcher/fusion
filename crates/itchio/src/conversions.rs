use super::api::models::{OwnedKey, Upload, UploadStorage};
use database::models::{Game, GameBuilder, GameSource};

impl From<OwnedKey> for Game {
    fn from(key: OwnedKey) -> Self {
        let developer = key
            .game
            .user
            .map(|user| user.display_name.unwrap_or(user.username));

        let cover_url = key.game.still_cover_url.or(key.game.cover_url);

        let mut builder =
            GameBuilder::new(key.game.id.to_string(), GameSource::Itchio, key.game.title)
                /*.key(key.id.to_string()) */;

        if let Some(developer_name) = developer {
            builder = builder.developer(developer_name);
        }

        if let Some(cover) = cover_url {
            builder = builder.cover_url(cover);
        }

        builder.build()
    }
}

/*impl From<Upload> for GameVersion {
    fn from(upload: Upload) -> Self {
        GameVersion {
            id: upload.id.to_string(),
            name: upload.display_name.unwrap_or(upload.filename),
            external: upload.storage == UploadStorage::External,
        }
    }
}*/
