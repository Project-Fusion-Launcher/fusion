const BASE_URL: &str = "https://api.legacygames.com";

/// Checks if a user exists by email.
pub fn is_exsists_by_email(email: &str) -> String {
    let url = format!("{}/users/isexistsbyemail?email={}", BASE_URL, email);
    url
}

/// Checks if a login is valid.
pub fn login() -> String {
    let url = format!("{}/users/login", BASE_URL);
    url
}

/// The list of giveaway games associated with the email.
pub fn get_giveaway_catalog_by_email(email: &str) -> String {
    let url = format!(
        "{}/users/getgiveawaycatalogbyemail?email={}",
        BASE_URL, email
    );
    url
}

/// The list of owned, purchased games.
pub fn get_user_downloads(user_id: u32) -> String {
    let url = format!("{}/users/downloads?userId={}", BASE_URL, user_id);
    url
}

/// The download url for a giveaway game.
pub fn get_giveaway_installer(installer_uuid: &str) -> String {
    let url = format!(
        "{}/products/giveawaydownload?installerUuid={}",
        BASE_URL, installer_uuid
    );
    url
}

/// The download url for a purchased game.
pub fn get_wp_installer(product_id: u32, game_id: &str) -> String {
    let url = format!(
        "{}/products/download?productId={}&gameId={}",
        BASE_URL, product_id, game_id
    );
    url
}
