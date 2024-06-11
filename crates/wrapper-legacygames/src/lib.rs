use api::models::IsExistsByEmail;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;

mod api;
mod tests;

pub struct LegacyGamesClient {
    email: String,
    token: Option<String>,
    http: reqwest::Client,
}

impl LegacyGamesClient {
    pub fn from_email(email: String) -> Self {
        Self {
            email,
            token: None,
            http: reqwest::Client::new(),
        }
    }

    pub fn from_token(email: String, token: String) -> Self {
        Self {
            email,
            token: Some(token),
            http: reqwest::Client::new(),
        }
    }

    async fn fetch_user_exists(&self) -> Result<IsExistsByEmail, reqwest::Error> {
        self.make_get_request(&api::endpoints::is_exsists_by_email(&self.email))
            .await
    }

    /// Makes a GET request to the Legacy Games API.
    async fn make_get_request<D>(&self, url: &str) -> Result<D, reqwest::Error>
    where
        D: DeserializeOwned,
    {
        let mut request = self
            .http
            .get(url)
            .header(AUTHORIZATION, "?token?")
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json");

        if self.token.is_some() {
            request = request.header(
                "UserToken",
                format!("Basic {}", self.token.clone().unwrap()),
            );
        }

        request.send().await?.json().await
    }
}
