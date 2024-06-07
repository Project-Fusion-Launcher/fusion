#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn owned_keys_test() {
        let owned_keys = owned_keys("abcd", 1).await;
        println!("{:?}", owned_keys);
        assert!(owned_keys.is_ok());
    }
}
