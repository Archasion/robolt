#![allow(dead_code)]

use serde::Deserialize;
use reqwest::Method;

use crate::utilities::client::{HTTP, HttpClientExt, HttpRequest};
use crate::web::ENDPOINTS;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticatedUser {
    #[serde(rename = "UserID")]
    pub id: u64,
    #[serde(rename = "UserName")]
    pub username: String,
    pub robux_balance: u64,
    pub thumbnail_url: String,
    pub is_premium: bool
}

pub fn login(cookie: &str) -> Result<(), &str> {
    let cookie = format!(".ROBLOSECURITY={cookie}");
    HTTP.set_cookie(&cookie)
}

pub fn logout() {
    HTTP.remove_cookie();
}

pub fn authenticated() -> Result<AuthenticatedUser, String> {
    let req = HttpRequest {
        method: Method::GET,
        url: format!("{}/mobileapi/userinfo", ENDPOINTS.web),
        headers: None,
        body: None,
    };

    HTTP.request::<AuthenticatedUser>(req)
}