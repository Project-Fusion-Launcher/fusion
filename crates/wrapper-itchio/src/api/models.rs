use chrono::NaiveDateTime;
use serde::{
    de::{DeserializeOwned, Error},
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Login {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub totp_needed: bool,
    pub token: Option<String>,
    #[serde(default)]
    pub recaptcha_needed: bool,
    pub recaptcha_url: Option<String>,
    pub cookie: Option<Cookie>,
    pub key: Option<Key>,
}

#[derive(Deserialize, Debug)]
pub struct Cookie {
    pub itchio: String,
}

#[derive(Deserialize, Debug)]
pub struct Key {
    pub key: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub updated_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub created_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub last_used_at: NaiveDateTime,
    pub source: KeySource,
    pub source_version: String,
    #[serde(default)]
    pub revoked: bool,
    pub user_id: u32,
    pub id: u32,
}

/// The login request body
#[derive(Serialize, Debug)]
pub struct LoginParams {
    pub source: &'static str,
    pub username: String,
    pub password: String,
    pub recaptcha_response: Option<String>,
}

/// The totp login request body
#[derive(Serialize, Debug)]
pub struct TOTPLoginParams {
    pub token: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct OwnedKeys {
    pub per_page: u8,
    pub page: u32,
    pub owned_keys: Vec<OwnedKey>,
}

#[derive(Debug, Deserialize)]
pub struct OwnedKey {
    pub id: u32,
    pub game_id: u32,
    pub purchase_id: Option<u32>,
    pub downloads: u32,
    #[serde(deserialize_with = "deserialize_date")]
    pub updated_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub created_at: NaiveDateTime,
    pub game: Game,
}

#[derive(Debug, Deserialize)]
pub struct Game {
    pub id: u32,
    pub url: String,
    pub title: String,
    pub short_text: Option<String>,
    #[serde(rename = "type")]
    pub r#type: GameType,
    pub classification: GameClassification,
    pub embed: Option<GameEmbedData>,
    pub cover_url: Option<String>,
    pub still_cover_url: Option<String>,
    #[serde(deserialize_with = "deserialize_date")]
    pub published_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub created_at: NaiveDateTime,
    pub min_price: u32,
    pub user: Option<User>,
    pub sale: Option<Sale>,
    #[serde(deserialize_with = "deserialize_empty_object")]
    pub traits: Vec<GameTraits>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub url: String,
    pub cover_url: Option<String>,
    pub still_cover_url: Option<String>,
    pub display_name: Option<String>,
    #[serde(default)]
    pub developer: bool,
    #[serde(default)]
    pub press_user: bool,
}

#[derive(Debug, Deserialize)]
pub struct Sale {
    pub id: u32,
    pub rate: u8,
    #[serde(deserialize_with = "deserialize_date")]
    pub start_date: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub end_date: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct GameEmbedData {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

#[derive(Debug, Deserialize)]
pub struct Uploads {
    #[serde(deserialize_with = "deserialize_empty_object")]
    pub uploads: Vec<Upload>,
}

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub upload: Upload,
}

#[derive(Debug, Deserialize)]
pub struct Upload {
    pub id: u32,
    pub game_id: u32,
    pub position: u32,
    pub storage: UploadStorage,
    pub host: Option<String>,
    pub filename: String,
    pub display_name: Option<String>,
    pub size: Option<u32>,
    pub channel_name: Option<String>,
    pub build: Option<Build>,
    pub build_id: Option<u32>,
    pub md5_hash: Option<String>,
    #[serde(rename = "type")]
    pub r#type: UploadType,
    #[serde(deserialize_with = "deserialize_date")]
    pub created_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub updated_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_empty_object")]
    pub traits: Vec<UploadTraits>,
}

#[derive(Debug, Deserialize)]
pub struct Builds {
    pub builds: Vec<Build>,
}

#[derive(Debug, Deserialize)]
pub struct BuildResponse {
    pub build: Build,
}

#[derive(Debug, Deserialize)]
pub struct Build {
    pub id: u32,
    pub upload_id: Option<u32>,
    #[serde(default)]
    pub parent_build_id: u32,
    pub state: Option<BuildState>,
    pub version: u32,
    pub user_version: Option<String>,
    pub user: Option<User>,
    #[serde(default)]
    pub files: Vec<BuildFile>,
    #[serde(deserialize_with = "deserialize_date")]
    pub created_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct BuildFile {
    pub size: u32,
    pub state: BuildFileState,
    #[serde(rename = "type")]
    pub r#type: BuildFileType,
    pub sub_type: BuildFileSubType,
}

#[derive(Deserialize, Debug)]
pub struct CollectionsResponse {
    pub collections: Vec<Collection>,
}

#[derive(Deserialize, Debug)]
pub struct Collection {
    pub id: u32,
    pub title: String,
    pub games_count: u32,
    #[serde(deserialize_with = "deserialize_date")]
    pub created_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct CollectionGames {
    pub page: u32,
    pub per_page: u8,
    pub collection_games: Vec<CollectionGame>,
}

#[derive(Deserialize, Debug)]
pub struct CollectionGame {
    pub blurb: Option<String>,
    pub position: u32,
    pub user_id: u32,
    #[serde(deserialize_with = "deserialize_date")]
    pub created_at: NaiveDateTime,
    pub game: Game,
}

#[derive(Deserialize, Debug)]
pub struct ScannedArchiveResponse {
    pub scanned_archive: ScannedArchive,
}

#[derive(Deserialize, Debug)]
pub struct ScannedArchive {
    pub object_id: u32,
    pub extracted_size: Option<u32>,
    #[serde(deserialize_with = "deserialize_date")]
    pub created_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub updated_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_empty_object")]
    pub launch_targets: Vec<LaunchTarget>,
}

#[derive(Deserialize, Debug)]
pub struct LaunchTarget {
    pub arch: String,
    pub flavor: String,
    pub path: String,
    pub sha256: String,
    pub size: u32,
    pub pe_info: PeInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PeInfo {
    pub imports: Vec<String>,
    pub arch: String,
    pub assembly_info: Option<AssemblyInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssemblyInfo {
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum KeySource {
    Desktop,
    Web,
    Wharf,
    Android,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GameType {
    Default,
    Flash,
    Unity,
    Java,
    Html,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UploadStorage {
    Hosted,
    Build,
    External,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GameClassification {
    Game,
    Tool,
    Assets,
    GameMod,
    PhysicalGame,
    Soundtrack,
    Other,
    Comic,
    Book,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GameTraits {
    PWindows,
    POsx,
    PLinux,
    PAndroid,
    CanBeBought,
    InPressSystem,
    HasDemo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UploadTraits {
    PWindows,
    POsx,
    PLinux,
    PAndroid,
    Preorder,
    Demo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UploadType {
    Default,
    Flash,
    Unity,
    Java,
    Html,
    Soundtrack,
    Book,
    Video,
    Documentation,
    Mod,
    AudioAssets,
    GraphicalAssets,
    Sourcecode,
    Other,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BuildState {
    Started,
    Processing,
    Completed,
    Failed,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BuildFileState {
    Created,
    Uploading,
    Uploaded,
    Failed,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BuildFileType {
    Patch,
    Archive,
    Signature,
    Manifest,
    Unpacked,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BuildFileSubType {
    Default,
    Gzip,
    Optimized,
}

// Deserialize date strings into actual dates
fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%fZ").map_err(Error::custom)
}

// Deserialize empty objects as vecs. For some reason some fields can be an empty object ({}) instead of an array.
fn deserialize_empty_object<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    match value {
        Value::Array(vec) => {
            Ok(serde_json::from_value(Value::Array(vec)).unwrap_or_else(|_| Vec::new()))
        }
        Value::Object(_) => Ok(Vec::new()),
        _ => Err(serde::de::Error::custom("Invalid data type for traits")),
    }
}
