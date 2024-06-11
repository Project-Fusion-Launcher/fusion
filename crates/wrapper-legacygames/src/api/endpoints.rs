const BASE_URL: &str = "https://api.legacygames.com";

/// Checks if a user exists by email.
pub fn is_exsists_by_email(email: &str) -> String {
    let url = format!("{}/users/isexistsbyemail?email={}", BASE_URL, email);
    url
}
