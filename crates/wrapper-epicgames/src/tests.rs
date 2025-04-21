#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn from_refresh_token() {
        let client = EpicGamesClient::from_refresh_token("asdf").await;
        assert!(
            client.is_ok(),
            "Failed to create client from refresh token: {:?}",
            client.err()
        );
    }

    #[tokio::test]
    async fn fetch_games() {
        let client = EpicGamesClient::from_access_token("asdf").await;
        let games = client.unwrap().fetch_games().await.unwrap();

        for game in games.iter().take(10) {
            println!("{:?}", game);
        }
    }
}
