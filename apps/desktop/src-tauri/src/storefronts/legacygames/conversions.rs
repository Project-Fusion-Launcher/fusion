use super::api::models::Product;
use crate::models::game::{Game, GameSource, GameStatus};

impl From<Product> for Vec<Game> {
    fn from(product: Product) -> Self {
        product
            .games
            .into_iter()
            .map(|game| {
                let (game_id, product_id) = if product.is_giveaway {
                    (game.installer_uuid.to_string(), None)
                } else {
                    (game.game_id.to_string(), Some(product.id.to_string()))
                };

                Game {
                    id: game_id,
                    title: game.game_name.clone(),
                    source: GameSource::LegacyGames,
                    key: product_id,
                    developer: None,
                    launch_target: None,
                    path: None,
                    version: None,
                    status: GameStatus::NotInstalled,
                    favorite: false,
                    hidden: false,
                    cover_url: Some(game.game_coverart),
                    sort_title: game.game_name.to_lowercase(),
                }
            })
            .collect()
    }
}
