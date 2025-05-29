use super::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AccessResponse {
    Success(AccessData),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize)]
pub struct AccessData {
    pub access_token: String,
    pub refresh_token: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AssetsResponse {
    Success(Vec<Asset>),
    Error(ErrorResponse),
}

#[derive(Debug)]
pub enum GameInfoResponse {
    Game(Box<Game>),
    Error(ErrorResponse),
}

impl<'de> Deserialize<'de> for GameInfoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: HashMap<String, serde_json::Value> = HashMap::deserialize(deserializer)?;
        if let Some((_, value)) = map.iter().next() {
            if let Ok(game) = serde_json::from_value::<Game>(value.clone()) {
                return Ok(GameInfoResponse::Game(Box::new(game)));
            }
            if let Ok(err) = serde_json::from_value::<ErrorResponse>(value.clone()) {
                return Ok(GameInfoResponse::Error(err));
            }
        }
        Err(Error::custom(
            "Could not deserialize into Game or ErrorResponse",
        ))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameManifestResponse {
    Elements(Vec<GameManifestElement>),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error_code: String,
}
