#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn is_exists_test() {
        let exists = LegacyGamesClient::fetch_user_exists("abcd").await;
        println!("{:?}", exists);
        assert!(exists.is_ok());
    }

    #[tokio::test]
    async fn login_test() {
        let token = LegacyGamesClient::generate_token("ABCD", "abcd");
        let login = LegacyGamesClient::test_login(token).await;
        println!("{:?}", login);
        assert!(login.is_ok());
    }

    #[tokio::test]
    async fn giveaway_products_test() {
        let client = LegacyGamesClient::from_email(String::from("abcd"));
        let products = client.fetch_giveaway_products().await;
        println!("{:?}", products);
        assert!(products.is_ok());
    }

    #[tokio::test]
    async fn products_test() {
        let token = LegacyGamesClient::generate_token("ABCD", "abcd");
        let mut client = LegacyGamesClient::from_token(String::from("abcd"), token);
        let products = client.fetch_wp_products().await;
        println!("{:?}", products);
        assert!(products.is_ok());
    }
}
