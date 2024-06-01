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
    /// The title of the game.
    pub title: String,
    /// The cover image of the game.
    pub cover: String,
    /// The year the game was published.
    pub year: u16,
}

#[derive(Debug, Serialize, Deserialize)]
/// Represents a game page.
pub struct GamePage {
    pub game: Game,
}

#[derive(Debug, Serialize, Deserialize)]
/// Represents a game.
pub struct Game {
    /// The unique identifier of the game.
    pub id: String,
    /// The title of the game.
    pub title: String,
    /// The cover image of the game.
    pub cover: String,
    /// The year the game was published.
    pub year: u16,
}
