use crate::{
    common::error::Result,
    managers::download::{Download, DownloadOptions},
    models::game::{Game, GameSource, GameStatus, GameVersion, VersionDownloadInfo},
};
use wrapper_legacygames::LegacyGamesClient;

pub async fn fetch_games(email: String, token: Option<String>) -> Result<Vec<Game>> {
    let mut client = if let Some(token) = token {
        LegacyGamesClient::from_token(email, token)
    } else {
        LegacyGamesClient::from_email(email)
    };

    let giveaway_products = client.fetch_giveaway_products().await?;

    let mut games = Vec::new();
    for product in giveaway_products {
        for game in product.games {
            games.push(Game {
                // Use installer_uuid as game ids can be duplicated for some reason
                id: game.installer_uuid,
                title: game.game_name,
                source: GameSource::LegacyGames,
                key: None,
                developer: None,
                launch_target: None,
                path: None,
                version: None,
                status: GameStatus::NotInstalled,
            });
        }
    }

    if client.is_token_client() {
        let wp_games = client.fetch_products().await?;

        for product in wp_games {
            for game in product.games {
                games.push(Game {
                    id: game.game_id,
                    title: game.game_name,
                    source: GameSource::LegacyGames,
                    key: Some(product.product_id.to_string()),
                    developer: None,
                    launch_target: None,
                    path: None,
                    version: None,
                    status: GameStatus::NotInstalled,
                });
            }
        }
    }

    Ok(games)
}

pub async fn fetch_releases(
    email: String,
    token: Option<String>,
    game: Game,
) -> Result<Vec<GameVersion>> {
    let client = if let Some(token) = token {
        LegacyGamesClient::from_token(email, token)
    } else {
        LegacyGamesClient::from_email(email)
    };

    let size = if game.key.is_none() {
        client.fetch_giveaway_installer_size(&game.id).await?
    } else {
        client
            .fetch_wp_installer_size(game.key.unwrap().parse()?, &game.id)
            .await?
    };

    Ok(vec![GameVersion {
        id: game.id.clone(),
        game_id: game.id.clone(),
        source: GameSource::LegacyGames,
        name: game.title.clone(),
        download_size: size,
    }])
}

pub fn fetch_release_info() -> VersionDownloadInfo {
    VersionDownloadInfo { install_size: 0 }
}

pub async fn fetch_download_info(
    email: String,
    token: Option<String>,
    game: &mut Game,
    download_options: DownloadOptions,
) -> Result<Download> {
    let client = if let Some(token) = token {
        LegacyGamesClient::from_token(email, token)
    } else {
        LegacyGamesClient::from_email(email)
    };

    let download_request = if game.key.is_none() {
        client.fetch_giveaway_installer(&game.id).await?
    } else {
        client
            .fetch_wp_installer(game.key.clone().unwrap().parse()?, &game.id)
            .await?
    };

    game.version = Some(game.id.clone());

    game.path = Some(
        download_options
            .install_location
            .to_string_lossy()
            .into_owned(),
    );

    Ok(Download {
        request: download_request,
        file_name: game.title.clone().replace(" ", "_").replace(":", " - ") + ".exe",
        download_options,
        source: GameSource::Itchio,
        game_id: game.id.clone(),
    })
}
