use crate::api::*;

/// Obtain an access token for the Epic Games account.
pub fn oauth_token() -> String {
    format!("{}/account/api/oauth/token", ACCOUNT_HOST)
}

/// List of assets owned.
pub fn assets(platform: &str, label: &str) -> String {
    format!(
        "{}/launcher/api/public/assets/{}?label={}",
        LAUNCHER_HOST, platform, label
    )
}

/// Get information about a game.
pub fn game_info(namespace: &str, catalog_item_id: &str) -> String {
    format!(
        "{}/catalog/api/shared/namespace/{}/bulk/items?id={}&includeDLCDetails=true&includeMainGameDetails=true&country=US&locale=en",
        CATALOG_HOST, namespace, catalog_item_id
    )
}

/// Get the game manifest info and dowload links for a specific game.
pub fn game_manifest(
    platform: &str,
    namespace: &str,
    catalog_item_id: &str,
    app_name: &str,
    label: &str,
) -> String {
    format!(
        "{}/launcher/api/public/assets/v2/platform/{}/namespace/{}/catalogItem/{}/app/{}/label/{}",
        LAUNCHER_HOST, platform, namespace, catalog_item_id, app_name, label
    )
}
