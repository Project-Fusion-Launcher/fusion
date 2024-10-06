const BASE_URL: &str = "https://api.itch.io";

/// The list of keys (games) that the user owns.
pub fn owned_keys(page: u32) -> String {
    let url = format!("{}/profile/owned-keys?page={}", BASE_URL, page);
    url
}

/// The list of uploads for a game.
pub fn uploads(game_id: u32, download_key_id: u32) -> String {
    let url = format!(
        "{}/games/{}/uploads?download_key_id={}",
        BASE_URL, game_id, download_key_id
    );
    url
}

/// A specific upload.
pub fn upload(upload_id: u32, download_key_id: u32) -> String {
    let url = format!(
        "{}/uploads/{}?download_key_id={}",
        BASE_URL, upload_id, download_key_id
    );
    url
}

/// Information about the scanned archive of an upload.
pub fn upload_scanned_archive(upload_id: u32, download_key_id: u32) -> String {
    let url = format!(
        "{}/uploads/{}/scanned-archive?download_key_id={}",
        BASE_URL, upload_id, download_key_id
    );
    url
}

/// The download URL for an upload.
pub fn upload_download(upload_id: u32, download_key_id: u32) -> String {
    let url = format!(
        "{}/uploads/{}/download?download_key_id={}",
        BASE_URL, upload_id, download_key_id
    );
    url
}

/// The list of builds associated to an upload.
pub fn builds(upload_id: u32, download_key_id: u32) -> String {
    let url = format!(
        "{}/uploads/{}/builds?download_key_id={}",
        BASE_URL, upload_id, download_key_id
    );
    url
}

/// A specific build.
pub fn build(build_id: u32, download_key_id: u32) -> String {
    let url = format!(
        "{}/builds/{}?download_key_id={}",
        BASE_URL, build_id, download_key_id
    );
    url
}

/// Information about the scanned archive of a build.
pub fn build_scanned_archive(build_id: u32, download_key_id: u32) -> String {
    let url = format!(
        "{}/builds/{}/scanned-archive?download_key_id={}",
        BASE_URL, build_id, download_key_id
    );
    url
}

/// The list of collections that the user has created.
pub fn collections() -> String {
    let url = format!("{}/profile/collections", BASE_URL);
    url
}

/// The list of games in a collection.
pub fn collection_games(collection_id: u32, page: u32) -> String {
    let url = format!(
        "{}/collections/{}/collection-games?page={}",
        BASE_URL, collection_id, page
    );
    url
}

/// Obtain API key with username/email and password.
/// A TOTP code response may be returned instead.
/// This is a POST request.
pub fn login() -> String {
    let url = format!("{}/login", BASE_URL);
    url
}

/// Verify a TOTP code to obtain an API key.
pub fn totp_verify() -> String {
    let url = format!("{}/totp/verify", BASE_URL);
    url
}
