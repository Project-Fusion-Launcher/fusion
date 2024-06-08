#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn owned_keys_test() {
        let client = ItchioClient::new("abcd");
        let owned_keys = client.fetch_owned_keys(1).await;
        println!("{:?}", owned_keys);
        assert!(owned_keys.is_ok());
    }

    #[tokio::test]
    async fn uploads_test() {
        let client = ItchioClient::new("abcd");
        let uploads = client.fetch_game_uploads(204750, 94235473).await;
        println!("{:?}", uploads);
        assert!(uploads.is_ok());
    }

    #[tokio::test]
    async fn upload_test() {
        let client = ItchioClient::new("abcd");
        let upload = client.fetch_game_upload(706309, 94235473).await;
        println!("{:?}", upload);
        assert!(upload.is_ok());
    }

    #[tokio::test]
    async fn builds_test() {
        let client = ItchioClient::new("abcd");
        let builds = client.fetch_upload_builds(706309, 94235473).await;
        println!("{:?}", builds);
        assert!(builds.is_ok());
    }

    #[tokio::test]
    async fn build_test() {
        let client = ItchioClient::new("abcd");
        let build = client.fetch_upload_build(66666, 94235473).await;
        println!("{:?}", build);
        assert!(build.is_ok());
    }
}
