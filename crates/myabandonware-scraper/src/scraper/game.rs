use scraper::{Html, Selector};

use crate::{
    models::{Game, GameRating},
    routes::route_game,
};

pub async fn game(id: &str) -> Result<Game, &'static str> {
    let response = reqwest::get(route_game(id))
        .await
        .map_err(|_| "Failed to fetch search results")?
        .text()
        .await
        .map_err(|_| "Failed to read search results")?;

    let document = Html::parse_document(&response);

    let title_selector = Selector::parse("h2").unwrap();
    let game_info_selector = Selector::parse("table.gameInfo tbody tr").unwrap();
    let game_info_type_selector = Selector::parse("th").unwrap();
    let game_info_data_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let rating_selector = Selector::parse("div.gameRated").unwrap();

    let name = document
        .select(&title_selector)
        .next()
        .map(|name| name.text().collect())
        .unwrap_or_default();

    let mut alt_names = Vec::new();
    let mut year = 0;
    let mut publishers = Vec::new();
    let mut developer = String::new();

    for table_row in document.select(&game_info_selector) {
        let row_name = table_row
            .select(&game_info_type_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();

        match row_name.as_str() {
            "Alt names" => {
                alt_names = table_row
                    .select(&game_info_data_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();
            }
            "Year" => {
                year = table_row
                    .select(&a_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .parse()
                    .unwrap();
            }
            "Publisher" => {
                publishers = table_row
                    .select(&a_selector)
                    .map(|a| a.text().collect())
                    .collect();
            }
            "Developer" => {
                developer = table_row
                    .select(&a_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect();
            }
            _ => {}
        }
    }

    let rating = document
        .select(&rating_selector)
        .next()
        .map(|rating| {
            let rating_text = rating.text().collect::<String>();
            let rating_components = rating_text.split_whitespace().collect::<Vec<&str>>();
            let value = rating_components[0].parse().unwrap();
            let max = rating_components[2].parse().unwrap();
            let votes = rating_components[4].parse().unwrap();

            GameRating { value, max, votes }
        })
        .unwrap_or_default();

    Ok(Game {
        id: id.to_string(),
        name,
        alt_names,
        year,
        publishers,
        developer,
        rating,
    })
}
