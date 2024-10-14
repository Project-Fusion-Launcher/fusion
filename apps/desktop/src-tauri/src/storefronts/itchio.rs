use wrapper_itchio::ItchioClient;

use crate::{
    managers::download::{Download, DownloadOptions},
    models::game::{Game, GameVersion, VersionDownloadInfo},
};

pub async fn fetch_games(api_key: &str) -> Vec<Game> {
    let client = ItchioClient::new(api_key);

    let owned_keys = client.fetch_owned_keys(1).await.unwrap();

    let mut games = Vec::new();
    for key in owned_keys.owned_keys {
        let developer = key
            .game
            .user
            .and_then(|user| user.display_name.or(Some(user.username)));

        games.push(Game {
            id: key.game.id.to_string(),
            title: key.game.title,
            source: "itchio".to_string(),
            key: Some(key.id.to_string()),
            developer,
            launch_target: None,
            path: None,
        });
    }

    games
}

pub async fn fetch_releases(api_key: &str, game_id: &str, game_key: &str) -> Vec<GameVersion> {
    let client = ItchioClient::new(api_key);

    let game_id: u32 = game_id.parse().unwrap();
    let game_key: u32 = game_key.parse().unwrap();
    let game = client.fetch_game_uploads(game_id, game_key).await.unwrap();

    game.uploads
        .into_iter()
        .map(|upload| GameVersion {
            id: upload.id.to_string(),
            game_id: game_id.to_string(),
            source: "itchio".to_string(),
            name: upload.display_name.unwrap_or(upload.filename),
            download_size: upload.size.unwrap_or(0),
        })
        .collect()
}

pub async fn fetch_release_info(api_key: &str, upload_id: &str, game: Game) -> VersionDownloadInfo {
    let client = ItchioClient::new(api_key);

    let upload_id: u32 = upload_id.parse().unwrap();
    let game_key: u32 = game.key.clone().unwrap().parse().unwrap();

    let mut retries = 5;

    while retries > 0 {
        let scanned_archive = client
            .fetch_upload_scanned_archive(upload_id, game_key)
            .await;

        if let Ok(scanned_archive) = scanned_archive {
            if scanned_archive.extracted_size.is_some() {
                return VersionDownloadInfo {
                    install_size: scanned_archive.extracted_size.unwrap(),
                };
            }
        }

        retries -= 1;

        tokio::time::sleep(std::time::Duration::from_secs(2_u64.pow(5 - retries))).await;
    }

    unimplemented!()
}

pub async fn fetch_download_info(
    api_key: &str,
    upload_id: &str,
    game: Game,
    download_options: DownloadOptions,
) -> Download {
    let client = ItchioClient::new(api_key);

    let upload_id: u32 = upload_id.parse().unwrap();
    let game_key: u32 = game.key.clone().unwrap().parse().unwrap();

    let upload = client.fetch_game_upload(upload_id, game_key).await.unwrap();

    let download_request = client
        .fetch_upload_download_url(upload_id, game_key)
        .await
        .unwrap();

    Download {
        request: download_request,
        filename: upload.filename,
        download_options,
    }
}
