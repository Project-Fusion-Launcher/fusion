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

    #[tokio::test]
    async fn fetch_versions() {
        let client = EpicGamesClient::from_access_token("asdf").await;
        let versions = client
            .unwrap()
            .fetch_game_versions("d5326cb42d704158bab2dc1629295838")
            .await
            .unwrap();

        println!("{:?}", versions);
    }
}
