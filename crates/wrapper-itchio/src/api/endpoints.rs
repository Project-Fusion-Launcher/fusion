const BASE_URL: &str = "https://api.itch.io";

/// The list of keys (games) that the user owns.
pub fn owned_keys(page: u32) -> String {
    format!("{}/profile/owned-keys?page={}", BASE_URL, page)
}

/// The list of uploads for a game.
pub fn uploads(game_id: u32, download_key_id: u32) -> String {
    format!(
        "{}/games/{}/uploads?download_key_id={}",
        BASE_URL, game_id, download_key_id
    )
}

/// A specific upload.
pub fn upload(upload_id: u32, download_key_id: u32) -> String {
    format!(
        "{}/uploads/{}?download_key_id={}",
        BASE_URL, upload_id, download_key_id
    )
}

/// Information about the scanned archive of an upload.
pub fn upload_scanned_archive(upload_id: u32, download_key_id: u32) -> String {
    format!(
        "{}/uploads/{}/scanned-archive?download_key_id={}",
        BASE_URL, upload_id, download_key_id
    )
}

/// The download URL for an upload.
pub fn upload_download(upload_id: u32, download_key_id: u32) -> String {
    format!(
        "{}/uploads/{}/download?download_key_id={}",
        BASE_URL, upload_id, download_key_id
    )
}

/// The list of builds associated to an upload.
pub fn builds(upload_id: u32, download_key_id: u32) -> String {
    format!(
        "{}/uploads/{}/builds?download_key_id={}",
        BASE_URL, upload_id, download_key_id
    )
}

/// A specific build.
pub fn build(build_id: u32, download_key_id: u32) -> String {
    format!(
        "{}/builds/{}?download_key_id={}",
        BASE_URL, build_id, download_key_id
    )
}

/// Information about the scanned archive of a build.
pub fn build_scanned_archive(build_id: u32, download_key_id: u32) -> String {
    format!(
        "{}/builds/{}/scanned-archive?download_key_id={}",
        BASE_URL, build_id, download_key_id
    )
}

/// The list of collections that the user has created.
pub fn collections() -> String {
    format!("{}/profile/collections", BASE_URL)
}

/// The list of games in a collection.
pub fn collection_games(collection_id: u32, page: u32) -> String {
    format!(
        "{}/collections/{}/collection-games?page={}",
        BASE_URL, collection_id, page
    )
}

/// Obtain API key with username/email and password.
/// A TOTP code response may be returned instead.
/// This is a POST request.
pub fn login() -> String {
    format!("{}/login", BASE_URL)
}

/// Verify a TOTP code to obtain an API key.
pub fn totp_verify() -> String {
    format!("{}/totp/verify", BASE_URL)
}
