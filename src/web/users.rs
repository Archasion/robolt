#![allow(unused)]

use reqwest::Method;
use serde::Deserialize;

use crate::utilities::client::{HttpClientExt, HttpRequest, HTTP};
use crate::web::ENDPOINTS;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "name")]
    pub username: String,
    pub display_name: String,
    pub is_banned: bool,
    pub description: String,
    pub created: String,
    pub has_verified_badge: bool,
    pub external_app_display_name: Option<String>,
    pub id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PartialUser {
    pub username: String,
    pub id: u64,
}

/// Returns a [`User`] struct containing information about the user with the given ID.
///
/// # Arguments
///
/// * `id` - The ID of the user to fetch.
///
/// # Examples
///
/// ```rust
/// use arlox::web::users;
///
/// let user = users::fetch(1);
/// println!("{:?}", user);
/// ```
pub fn fetch(id: u64) -> Result<User, String> {
    let req = HttpRequest {
        method: Method::GET,
        url: format!("{}/v1/users/{}", ENDPOINTS.users, id),
        headers: None,
        body: None,
    };

    HTTP.request(req)
}

/// Returns a [`PartialUser`] struct, only containing the username and ID
/// of the fetched user
///
/// # Arguments
///
/// * `username` - The username of the user to fetch.
///
/// # Examples
///
/// ```rust
/// use arlox::web::users;
///
/// let user = users::partial(1);
/// println!("{:?}", user);
/// ```
pub fn partial(id: u64) -> Result<PartialUser, String> {
    let req = HttpRequest {
        method: Method::GET,
        url: format!("{}/users/{}", ENDPOINTS.base, id),
        headers: None,
        body: None,
    };

    HTTP.request(req)
}

pub fn find(username: &str) -> Result<PartialUser, String> {
    let req = HttpRequest {
        method: Method::GET,
        url: format!(
            "{}/users/get-by-username?username={}",
            ENDPOINTS.base, username
        ),
        headers: None,
        body: None,
    };

    HTTP.request(req)
}

impl PartialUser {
    pub fn fetch(&self) -> Result<User, String> {
        fetch(self.id)
    }
}
