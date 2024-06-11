use serde::Deserialize;

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
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    Error,
}
