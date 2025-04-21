pub fn access_token() -> &'static str {
    "https://account-public-service-prod03.ol.epicgames.com/account/api/oauth/token"
}

pub fn assets() -> &'static str {
    "https://launcher-public-service-prod06.ol.epicgames.com/launcher/api/public/assets/Windows?label=Live"
}

pub fn game_info(namespace: &str, catalog_item_id: &str) -> String {
    format!("https://catalog-public-service-prod06.ol.epicgames.com/catalog/api/shared/namespace/{}/bulk/items?id={}&includeDLCDetails=true&includeMainGameDetails=true&country=US&locale=en", namespace, catalog_item_id)
}
