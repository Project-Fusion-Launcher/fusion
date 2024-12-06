use crate::{
    common::result::Result,
    managers::download::{Download, DownloadOptions},
    models::game::{Game, GameSource, GameStatus, GameVersion, VersionDownloadInfo},
};
use std::sync::Arc;
use tokio::task::JoinSet;
use wrapper_legacygames::{api::models::Product, LegacyGamesClient};

pub async fn fetch_games(email: String, token: Option<String>) -> Result<Vec<Game>> {
    let client = match token {
        Some(token) => Arc::new(LegacyGamesClient::from_token(email, token)),
        None => Arc::new(LegacyGamesClient::from_email(email)),
    };

    let mut join_set = JoinSet::new();

    if client.is_token_client() {
        let client_clone = client.clone();

        join_set.spawn(async move {
            match client_clone.fetch_wp_products().await {
                Ok(products) => {
                    let games = create_games(products, false);
                    Ok(games)
                }
                Err(err) => Err(err),
            }
        });
    }

    join_set.spawn(async move {
        match client.fetch_giveaway_products().await {
            Ok(products) => {
                let games = create_games(products, true);
                Ok(games)
            }
            Err(err) => Err(err),
        }
    });

    let mut result = Vec::new();

    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(games) => result.extend(games?),
            Err(e) => return Err(e.into()),
        }
    }

    Ok(result)
}

pub async fn fetch_game_versions(
    email: String,
    token: Option<String>,
    game: Game,
) -> Result<Vec<GameVersion>> {
    let client = match token {
        Some(token) => LegacyGamesClient::from_token(email, token),
        None => LegacyGamesClient::from_email(email),
    };

    let size = match game.key {
        Some(ref key) => {
            client
                .fetch_wp_installer_size(key.parse()?, &game.id)
                .await?
        }
        None => client.fetch_giveaway_installer_size(&game.id).await?,
    };

    Ok(vec![GameVersion {
        id: game.id.clone(),
        game_id: game.id,
        source: GameSource::LegacyGames,
        name: game.title,
        download_size: size,
    }])
}

pub fn fetch_release_info() -> VersionDownloadInfo {
    // There is no way to fetch the installed size that I know.
    // The game_installed_size in the API's resonse is actually the download size.
    VersionDownloadInfo { install_size: 0 }
}

pub async fn fetch_download_info(
    email: String,
    token: Option<String>,
    game: &mut Game,
    download_options: DownloadOptions,
) -> Result<Download> {
    let client = match token {
        Some(token) => LegacyGamesClient::from_token(email, token),
        None => LegacyGamesClient::from_email(email),
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

fn create_games(products: Vec<Product>, is_giveaway: bool) -> Vec<Game> {
    products
        .into_iter()
        .flat_map(|product| {
            product.games.into_iter().map(move |game| {
                let (game_id, product_id) = if is_giveaway {
                    (game.installer_uuid.to_string(), None)
                } else {
                    (game.game_id.to_string(), Some(product.id.to_string()))
                };

                Game {
                    id: game_id,
                    title: game.game_name,
                    source: GameSource::LegacyGames,
                    key: product_id,
                    developer: None,
                    launch_target: None,
                    path: None,
                    version: None,
                    status: GameStatus::NotInstalled,
                }
            })
        })
        .collect()
}
