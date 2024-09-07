use wrapper_itchio::ItchioClient;

use crate::models::game::Game;

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
