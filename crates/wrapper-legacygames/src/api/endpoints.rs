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
