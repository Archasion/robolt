#![allow(dead_code)]

use serde::Deserialize;

use crate::client::{HTTP, HttpClientExt};
use crate::web::ENDPOINTS;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticatedUser {
    #[serde(rename = "UserId")]
    id: u64,
    #[serde(rename = "UserName")]
    username: String,
    robux_balance: u64,
    thumbnail_url: String,
    is_premium: bool
}

pub fn login(cookie: &str) -> Result<(), &str> {
    HTTP.set_cookie(cookie)
}

pub fn me() -> Result<AuthenticatedUser, String> {
    let url = format!("{}/mobileapi/userinfo", ENDPOINTS.web);
    HTTP.req::<AuthenticatedUser>(reqwest::Method::GET, &url, None)
}