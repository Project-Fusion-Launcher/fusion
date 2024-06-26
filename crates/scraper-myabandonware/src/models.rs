use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Represents a search result.
pub struct SearchResult {
    /// The current page index
    pub current_page: u32,
    /// The total number of pages
    pub total_pages: u32,
    /// The games in this page
    pub items: Vec<SearchItem>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Represents a search item (game with reduced information)
pub struct SearchItem {
    /// The unique identifier of the game.
    pub id: String,
    /// The name of the game.
    pub name: String,
    /// The cover image of the game.
    pub cover: String,
    /// The year the game was published.
    pub year: u16,
}

#[derive(Debug, Serialize, Deserialize)]
/// Represents a game.
pub struct Game {
    /// The unique identifier of the game.
    pub id: String,
    /// The name of the game.
    pub name: String,
    /// The alternative names of the game.
    pub alt_names: Vec<String>,
    /// The year the game was published.
    pub year: u16,
    /// The publishers of the game.
    pub publishers: Vec<String>,
    /// The developer of the game.
    pub developer: String,
    /// The rating of the game.
    pub rating: GameRating,
    /// The available downloads of the game.
    pub downloads: Vec<GameDownload>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
/// Represents a game rating.
pub struct GameRating {
    /// The average rating of the game.
    pub value: f32,
    /// The maximum rating of the game. (5.0 at the time of writing this)
    pub max: f32,
    /// The current amount of votes.
    pub votes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
/// Represents a game download.
pub struct GameDownload {
    /// The name of the download.
    pub name: String,
}
