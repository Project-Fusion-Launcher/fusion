use crate::storefronts::legacygames::api::BASE_URL;

/// Check if the user exists by email
pub fn user_exists_by_email(email: &str) -> String {
    format!("{}/users/isexistsbyemail?email={}", BASE_URL, email)
}

/// Check if the login is valid
pub fn user_login() -> String {
    format!("{}/users/login", BASE_URL)
}

/// The list of giveaway games associated with the email
pub fn giveaway_catalog_by_email(email: &str) -> String {
    format!(
        "{}/users/getgiveawaycatalogbyemail?email={}",
        BASE_URL, email
    )
}

/// The list of owned, purchased games
pub fn user_downloads(user_id: u32) -> String {
    format!("{}/users/downloads?userId={}", BASE_URL, user_id)
}

/// The download url for a giveaway game.
pub fn giveaway_installer(installer_uuid: &str) -> String {
    format!(
        "{}/products/giveawaydownload?installerUuid={}",
        BASE_URL, installer_uuid
    )
}

/// The download url for a purchased game.
pub fn wp_installer(product_id: u32, game_id: &str) -> String {
    format!(
        "{}/products/download?productId={}&gameId={}",
        BASE_URL, product_id, game_id
    )
}
