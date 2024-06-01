use scraper::{Html, Selector};

use crate::models::{SearchItem, SearchResult};
use crate::routes::{route_search, BASE_URL};

pub async fn search(query: &str, hide_sold: bool) -> Result<SearchResult, &'static str> {
    let response = reqwest::get(route_search(query, hide_sold))
        .await
        .map_err(|_| "Failed to fetch search results")?
        .text()
        .await
        .map_err(|_| "Failed to read search results")?;

    let document = Html::parse_document(&response);

    let game_selector = Selector::parse("div.c-item-game").unwrap();
    let name_selector = Selector::parse("a.c-item-game__name").unwrap();
    let cover_selector = Selector::parse("img.c-thumb__img").unwrap();
    let year_selector = Selector::parse("span.c-item-game__year").unwrap();

    let pagination_selector = Selector::parse("div.pagination").unwrap();
    let current_page_selector = Selector::parse("a.current").unwrap();

    let mut result = Vec::new();

    let mut current_page = 0;
    let mut total_pages = 0;

    for game in document.select(&game_selector) {
        let mut id = String::new();
        let mut title = String::new();
        let mut cover = String::new();
        let mut year = 0;
        for game_name in game.select(&name_selector) {
            id = game_name
                .attr("href")
                .unwrap()
                .to_string()
                .replace("/game/", "");
            title = game_name.text().collect();
        }

        for game_cover in game.select(&cover_selector) {
            cover = format!(
                "{}{}",
                BASE_URL,
                game_cover.attr("src").unwrap().to_string()
            );
        }

        for game_year in game.select(&year_selector) {
            year = game_year.text().collect::<String>().parse().unwrap();
        }

        result.push(SearchItem {
            id,
            title,
            cover,
            year,
        })
    }

    for pagination_select in document.select(&pagination_selector) {
        for current_page_select in pagination_select.select(&current_page_selector) {
            current_page = current_page_select
                .text()
                .collect::<String>()
                .parse()
                .unwrap();
        }

        total_pages = pagination_select
            .child_elements()
            .last()
            .unwrap()
            .text()
            .collect::<String>()
            .parse()
            .unwrap();
    }

    Ok(SearchResult {
        items: result,
        current_page,
        total_pages,
    })
}
