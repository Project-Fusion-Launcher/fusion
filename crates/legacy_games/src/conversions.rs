use super::api::models::Product;
use database::models::{Game, GameBuilder, GameSource};

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

                let mut builder =
                    GameBuilder::new(game_id, GameSource::LegacyGames, game.game_name)
                        .cover_url(game.game_coverart);

                if let Some(product_id) = product_id {
                    //builder = builder.key(product_id);
                }

                builder.build()
            })
            .collect()
    }
}
