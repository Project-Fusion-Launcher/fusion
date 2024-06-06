use scraper::selectable::Selectable;
use scraper::{Html, Selector};

use crate::models::{SearchItem, SearchResult};
use crate::routes::{route_search, BASE_URL};

pub async fn search(query: &str, hide_sold: bool, page: u32) -> Result<SearchResult, &'static str> {
    let response = reqwest::get(route_search(query, hide_sold, page))
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
        let id = game
            .select(&name_selector)
            .next()
            .map(|game_name| game_name.attr("href").unwrap().replace("/game/", ""))
            .unwrap_or_default();

        let name = game
            .select(&name_selector)
            .next()
            .map(|game_name| game_name.text().collect())
            .unwrap_or_default();

        let cover = game
            .select(&cover_selector)
            .next()
            .map(|game_cover| format!("{}{}", BASE_URL, game_cover.attr("src").unwrap()))
            .unwrap_or_default();

        let year = game
            .select(&year_selector)
            .next()
            .map(|game_year| game_year.text().collect::<String>().parse().unwrap())
            .unwrap_or_default();

        result.push(SearchItem {
            id,
            name,
            cover,
            year,
        });
    }

    if let Some(pagination_select) = document.select(&pagination_selector).next() {
        if let Some(current_page_select) = pagination_select.select(&current_page_selector).next() {
            current_page = current_page_select
                .text()
                .collect::<String>()
                .parse()
                .unwrap();
        }

        total_pages = pagination_select
            .child_elements()
            .last()
            .map(|last_element| last_element.text().collect::<String>().parse().unwrap())
            .unwrap_or_default();
    }

    Ok(SearchResult {
        items: result,
        current_page,
        total_pages,
    })
}
