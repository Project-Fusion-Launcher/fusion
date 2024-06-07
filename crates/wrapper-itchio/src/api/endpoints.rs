use super::models::OwnedKeys;

const BASE_URL: &str = "https://api.itch.io";

/// Get the list of keys (games) that the user owns.
pub async fn owned_keys<S: AsRef<str>>(api_key: S, page: u32) -> Result<OwnedKeys, reqwest::Error> {
    let url = format!("{}/profile/owned-keys", BASE_URL);

    let response: OwnedKeys = reqwest::Client::new()
        .get(url)
        .query(&[("page", page)])
        .header("Authorization", api_key.as_ref())
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
