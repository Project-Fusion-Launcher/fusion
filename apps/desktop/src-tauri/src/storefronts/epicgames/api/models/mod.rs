use chrono::NaiveDateTime;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

mod chunk;
pub use chunk::*;
mod json_manifest;
pub use json_manifest::*;
mod manifest;
pub use manifest::*;
mod responses;
pub use responses::*;
mod requests;
pub use requests::*;
mod utils;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub app_name: String,
    pub label_name: String,
    pub build_version: String,
    pub catalog_item_id: String,
    pub namespace: String,
    pub asset_id: String,
    pub metadata: Option<AssetMetadata>,
    pub sidecar_rvn: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetMetadata {
    pub installation_pool_id: Option<String>,
    pub update_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: String,
    pub title: String,
    pub description: String,
    pub key_images: Vec<KeyImage>,
    pub categories: Vec<Category>,
    pub namespace: String,
    pub status: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub creation_date: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub last_modified_date: NaiveDateTime,
    // pub custom_attributes
    pub entitlement_name: String,
    pub entitlement_type: String,
    pub item_type: String,
    pub release_info: Vec<ReleaseInfo>,
    pub developer: String,
    pub developer_id: String,
    #[serde(default)]
    pub eulda_ids: Vec<String>,
    // pub install_modes
    pub end_of_support: bool,
    // pub dlc_item_list
    pub main_game_item: Option<Box<Game>>,
    #[serde(default)]
    pub main_game_item_list: Vec<Game>,
    // pub age_gatings
    pub application_id: String,
    #[serde(default)]
    pub requires_secure_account: bool,
    pub unsearchable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub path: CategoryPath,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameManifestElement {
    pub app_name: String,
    pub label_name: String,
    pub build_version: String,
    pub hash: String,
    pub use_signed_url: bool,
    pub manifests: Vec<ManifestUrl>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestUrl {
    pub uri: String,
    pub query_params: Vec<QueryParam>,
}

#[derive(Debug, Deserialize)]
pub struct QueryParam {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CategoryPath {
    Public,
    Addons,
    Games,
    Applications,
    Editors,
    Engines,
    Developer,
    Software,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseInfo {
    pub id: String,
    pub app_id: String,
    pub platform: Vec<Platform>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyImage {
    #[serde(rename = "type")]
    pub image_type: KeyImageType,
    pub url: String,
    pub md5: String,
    pub width: u32,
    pub height: u32,
    pub size: u32,
    #[serde(deserialize_with = "deserialize_date")]
    pub uploaded_date: NaiveDateTime,
    pub alt: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum Platform {
    Windows,
    Win32,
    Mac,
    Android,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum KeyImageType {
    DieselGameBox,
    DieselGameBoxLogo,
    DieselGameBoxTall,
    DieselGameBoxWide,
    DieselStoreFrontTall,
    DieselStoreFrontWide,
    OfferImageTall,
    OfferImageWide,
    Thumbnail,
    #[serde(other)]
    Other,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%fZ").map_err(Error::custom)
}
