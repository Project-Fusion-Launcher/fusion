#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn search_test() {
        let games = search("CSI", false).await;
        assert!(games.is_ok());
    }

    #[tokio::test]
    async fn game_test() {
        let game_page = game("grand-theft-auto-3w6").await;
        assert!(game_page.is_ok());
    }
}