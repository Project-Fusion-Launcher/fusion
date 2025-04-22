use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(Serialize)]
pub struct LoginParams {
    pub grant_type: GrantType,
    pub token_type: String,
    pub code: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    #[serde(deserialize_with = "deserialize_date")]
    pub expires_at: NaiveDateTime,
    pub token_type: String,
    pub refresh_token: String,
    pub refresh_expires: u32,
    #[serde(deserialize_with = "deserialize_date")]
    pub refresh_expires_at: NaiveDateTime,
    pub account_id: String,
    pub client_id: String,
    pub internal_client: bool,
    pub client_service: String,
    // pub scopes: Vec<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub app: String,
    pub in_app_id: String,
    pub acr: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub auth_time: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetMetadata {
    pub installation_pool_id: Option<String>,
    pub update_type: Option<String>,
}

#[derive(Debug)]
pub struct GameInfoResponse {
    pub game: Game,
}

impl<'de> Deserialize<'de> for GameInfoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: HashMap<String, Game> = HashMap::deserialize(deserializer)?;

        let game = map
            .into_iter()
            .next()
            .ok_or_else(|| serde::de::Error::custom("expected at least one game"))?
            .1;

        Ok(GameInfoResponse { game })
    }
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
    // pub release_info
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
    pub application_id: Option<String>,
    #[serde(default)]
    pub requires_secure_account: bool,
    pub unsearchable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub path: CategoryPath,
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
    #[serde(rename = "games/edition")]
    GamesEdition,
    #[serde(rename = "games/edition/base")]
    GamesEditionBase,
    #[serde(rename = "freegames")]
    FreeGames,
    #[serde(rename = "asset-format")]
    AssetFormat,
    #[serde(rename = "asset-format/game-engine")]
    AssetFormatGameEngine,
    #[serde(rename = "asset-format/game-engine/unreal-engine")]
    AssetFormatGameEngineUnrealEngine,
    #[serde(rename = "digitalextras")]
    DigitalExtras,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "type/format-item")]
    TypeFormatItem,
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
    ESRB,
    Featured,
    AndroidIcon,
    #[serde(rename = "CodeRedemption_340x440")]
    CodeRedemption340x440,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%fZ").map_err(Error::custom)
}
