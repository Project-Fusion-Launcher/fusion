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
