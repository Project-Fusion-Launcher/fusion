use serde::{Deserialize, Deserializer};

mod responses;
pub use responses::*;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum WpUser {
    False,
    User { id: u32, user_login: String },
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GiveawayUser {
    False,
    User { status: Status },
}

#[derive(Deserialize, Debug)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub product_id: u32,
    pub games: Vec<Game>,
    #[serde(default)]
    pub is_giveaway: bool,
}

#[derive(Deserialize, Debug)]
pub struct Game {
    pub game_id: String,
    #[serde(deserialize_with = "deserialize_game_name")]
    pub game_name: String,
    pub game_description: String,
    pub game_coverart: String,
    pub game_installed_size: String,
    pub installer_uuid: String,
}

#[derive(Deserialize, Debug)]
pub struct Installer {
    pub file: String,
}

fn deserialize_game_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let game_name: String = Deserialize::deserialize(deserializer)?;
    if game_name.is_empty() {
        Ok(String::from("!Unknown Game"))
    } else {
        Ok(game_name)
    }
}
