pub const BASE_URL: &str = "https://www.myabandonware.com";

/// Search for games on MyAbandonware given a query.
pub fn route_search(query: &str, hide_sold: bool) -> String {
    format!(
        "{}/search/q/{}{}",
        BASE_URL,
        query,
        if hide_sold { "/hs/1" } else { "" }
    )
}

/// Get the game page on MyAbandonware given an ID.
pub fn route_game(id: &str) -> String {
    format!("{}/game/{}", BASE_URL, id)
}
