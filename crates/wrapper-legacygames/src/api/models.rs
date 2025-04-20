use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct IsExistsByEmail {
    pub status: Status,
    pub data: IsExistsByEmailData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IsExistsByEmailData {
    pub giveaway_user: GiveawayUser,
    pub wp_user: WpUser,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GiveawayUser {
    False(bool),
    User {
        status: Status,
        data: Vec<GiveawayUserData>,
    },
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum WpUser {
    False(bool),
    User {
        id: u32,
        user_login: String,
        nickname: String,
    },
}

#[derive(Deserialize, Debug)]
pub struct GiveawayUserData {
    pub product_id: String,
    pub game_id: String,
    pub installer_uuid: String,
    pub order_id: String,
}

#[derive(Deserialize, Debug)]
pub struct TestLogin {
    pub status: Status,
    pub data: TestLoginData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TestLoginData {
    pub code: Option<String>,
    pub message: Option<String>,
    pub user_id: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Products {
    pub status: Status,
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
    pub catalog_visibility: CatalogVisibility,
    pub games: Vec<Game>,
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
    pub status: Status,
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
    pub id: String,
    pub name: String,
    pub installer_uuid: String,
    pub file: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    Error,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CatalogVisibility {
    Hidden,
    Visible,
}

fn deserialize_game_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let game_name: String = Deserialize::deserialize(deserializer)?;
    if game_name.is_empty() {
        Ok(String::from("!noname"))
    } else {
        Ok(game_name)
    }
}
