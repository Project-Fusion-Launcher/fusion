use crate::storefronts::itchio::api::BASE_URL;

/// The list of keys (games) that the user owns.
pub fn owned_keys(page: u32) -> String {
    format!("{}/profile/owned-keys?page={}", BASE_URL, page)
}

/// The list of uploads for a game.
pub fn game_uploads(game_id: u32, download_key_id: u32) -> String {
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
