use crate::common::result::Result;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;

pub async fn get<D, U>(http: &reqwest::Client, url: U, api_key: &str) -> Result<D>
where
    D: DeserializeOwned,
    U: IntoUrl,
{
    Ok(http
        .get(url)
        .header("Authorization", api_key)
        .send()
        .await?
        .json()
        .await?)
}
