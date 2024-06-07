use chrono::NaiveDateTime;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json::Value;

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
    pub title: String,
    pub short_text: Option<String>,
    #[serde(deserialize_with = "deserialize_date")]
    pub published_at: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_date")]
    pub created_at: NaiveDateTime,
    pub url: String,
    pub cover_url: Option<String>,
    pub still_cover_url: Option<String>,
    pub min_price: u32,
    #[serde(rename = "type")]
    pub r#type: GameType,
    pub classification: GameClassification,
    pub user: Option<User>,
    pub sale: Option<Sale>,
    pub embed: Option<Embed>,
    #[serde(deserialize_with = "deserialize_traits")]
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
pub struct Embed {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GameType {
    Default,
    Flash,
    Unity,
    Java,
    Html,
}

#[derive(Deserialize, Clone, Debug)]
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

#[derive(Deserialize, Clone, Debug)]
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

// Deserialize date strings into actual dates
fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%fZ").map_err(Error::custom)
}

// Deserialize traits. For some reason sometimes it can be an empty object instead of an array.
fn deserialize_traits<'de, D>(deserializer: D) -> Result<Vec<GameTraits>, D::Error>
where
    D: Deserializer<'de>,
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
