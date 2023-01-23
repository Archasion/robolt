use serde::Deserialize;
use crate::client::HTTP;
use crate::web::ENDPOINTS;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "name")]
    username: String,
    display_name: String,
    id: u64,
    is_banned: bool,
    description: String,
    created: String,
    has_verified_badge: bool,
    external_app_display_name: Option<String>,
}

pub fn fetch(id: u64) -> Result<User, String> {
    let url = format!("{}/v1/users/{}", ENDPOINTS.users, id);
    HTTP.read().unwrap().req(reqwest::Method::GET, &url, None)
}