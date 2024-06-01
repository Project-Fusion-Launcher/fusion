use scraper::Html;

use crate::routes::route_game;

pub async fn game(id: &str) -> Result<(), &'static str> {
    let response = reqwest::get(route_game(id))
        .await
        .map_err(|_| "Failed to fetch search results")?
        .text()
        .await
        .map_err(|_| "Failed to read search results")?;

    let document = Html::parse_document(&response);

    Ok(())
}
