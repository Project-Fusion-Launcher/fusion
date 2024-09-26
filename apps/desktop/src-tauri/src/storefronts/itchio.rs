use wrapper_itchio::ItchioClient;

use crate::models::game::{Game, GameVersion};

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
        })
        .collect()
}
