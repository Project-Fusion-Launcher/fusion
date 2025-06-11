use super::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileResponse {
    User(User),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OwnedKeysResponse {
    OwnedKeys(OwnedKeys),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize)]
pub struct OwnedKeys {
    pub per_page: u8,
    pub owned_keys: Vec<OwnedKey>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UploadsResponse {
    Uploads(Vec<Upload>),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UploadResponse {
    Upload(Box<Upload>),
    Error(ErrorResponse),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ScannedArchiveResponse {
    ScannedArchive(ScannedArchive),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
}
