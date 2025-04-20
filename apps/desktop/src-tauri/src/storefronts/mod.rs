use crate::models::game::GameSource;
use itchio::Itchio;
use legacygames::LegacyGames;

pub mod itchio;
pub mod legacygames;
#[macro_use]
pub mod storefront;

register_stores! {
    GameSource::Itchio => Itchio,
    GameSource::LegacyGames => LegacyGames
}
