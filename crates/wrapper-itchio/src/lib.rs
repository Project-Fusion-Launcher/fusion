use api::{endpoints, models::OwnedKeys};

mod api;
mod tests;

pub async fn owned_keys<S: AsRef<str>>(api_key: S, page: u32) -> Result<OwnedKeys, reqwest::Error> {
    endpoints::owned_keys(api_key, page).await
}
