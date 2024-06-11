#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn owned_keys_test() {
        let client = LegacyGamesClient::from_email(String::from("asdf"));
        //println!("{:?}", owned_keys);
        // assert!(owned_keys.is_ok());
    }
}
