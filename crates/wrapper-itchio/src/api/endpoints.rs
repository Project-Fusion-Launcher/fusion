use serde_json::Value;

const BASE_URL: &str = "https://api.itch.io";

pub async fn owned_keys<S: AsRef<str>>(api_key: S, page: S) {
    let url = format!("{}/owned-keys", BASE_URL);

    let response: Value = reqwest::Client::new()
        .get(url)
        .query(&("page", page.as_ref()))
        .header("Authorization", api_key.as_ref())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
}
