use serde::Serialize;

#[derive(Serialize)]
pub struct LoginParams {
    pub grant_type: GrantType,
    pub token_type: &'static str,
    pub code: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
}
