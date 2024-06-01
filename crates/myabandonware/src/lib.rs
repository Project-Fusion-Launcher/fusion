use models::{GamePage, SearchResult};

mod scraper;
mod models;
mod routes;
mod tests;

pub async fn search(query: &str, hide_sold: bool) -> Result<SearchResult, ()> {
    Ok(scraper::search::search(query, hide_sold).await.unwrap())
}

pub async fn game(id: &str) -> Result<(), ()> {
    Ok(scraper::game::game(id).await.unwrap())
}