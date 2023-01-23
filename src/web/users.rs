#![allow(unused)]

use serde::Deserialize;

use crate::client::{HTTP, HttpClientExt};
use crate::web::ENDPOINTS;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "name")]
    username: String,
    display_name: String,
    is_banned: bool,
    description: String,
    created: String,
    has_verified_badge: bool,
    external_app_display_name: Option<String>,
    id: u64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PartialUser {
    username: String,
    id: u64
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
    let url = format!("{}/v1/users/{}", ENDPOINTS.users, id);
    HTTP.req(reqwest::Method::GET, &url, None)
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
    let url = format!("{}/users/{}", ENDPOINTS.base, id);
    HTTP.req(reqwest::Method::GET, &url, None)
}

pub fn find(username: &str) -> Result<PartialUser, String> {
    let url = format!("{}/users/get-by-username?username={}", ENDPOINTS.base, username);
    HTTP.req(reqwest::Method::GET, &url, None)
}

impl PartialUser {
    pub fn fetch(&self) -> Result<User, String> {
        fetch(self.id)
    }
}