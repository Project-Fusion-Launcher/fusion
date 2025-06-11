use super::*;
use serde::Deserialize;

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
