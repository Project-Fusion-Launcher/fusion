use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct IsExistsByEmail {
    pub data: IsExistsByEmailData,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum IsExistsByEmailData {
    Error(String),
    UserData(UserData),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub giveaway_user: GiveawayUser,
    pub wp_user: WpUser,
}

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
pub struct TestLogin {
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct Products {
    pub data: ProductsData,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ProductsData {
    Error(String),
    Products(Vec<Product>),
}

#[derive(Deserialize, Debug)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub product_id: u32,
    pub purchasable: bool,
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
pub struct InstallerResponse {
    pub data: InstallerResponseData,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum InstallerResponseData {
    Error(String),
    Installer(Installer),
}

#[derive(Deserialize, Debug)]
pub struct Installer {
    pub file: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    Error,
}

impl Status {
    pub fn is_success(&self) -> bool {
        matches!(self, Status::Ok)
    }
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
