#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn search_test() {
        let games = search("need for speed", false, 1).await;
        println!("{:?}", games);
        assert!(games.is_ok());
    }

    #[tokio::test]
    async fn game_test() {
        let game_page = game("need-for-speed-underground-2-ega").await;
        println!("{:?}", game_page);
        assert!(game_page.is_ok());
    }
}
