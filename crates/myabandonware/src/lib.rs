use models::{GamePage, SearchResult};

mod models;
mod routes;
mod scraper;
mod tests;

pub async fn search(query: &str, hide_sold: bool, page: u32) -> Result<SearchResult, ()> {
    Ok(scraper::search::search(query, hide_sold, page)
        .await
        .unwrap())
}

pub async fn game(id: &str) -> Result<(), ()> {
    Ok(scraper::game::game(id).await.unwrap())
}
