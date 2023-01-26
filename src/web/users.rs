#![allow(unused)]

use reqwest::Method;
use serde::Deserialize;

use crate::utilities::client::{HttpClientExt, HttpRequest, HTTP};
use crate::web::auth::AuthenticatedUser;
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchResult {
    #[serde(rename = "name")]
    pub username: String,
    pub display_name: String,
    pub id: u64,
    pub has_verified_badge: bool,
    pub previous_usernames: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Username {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Data<T> {
    data: Vec<T>,
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
/// use robolt::web::users;
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

    HTTP.request::<User>(req)
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
/// use robolt::web::users;
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

    HTTP.request::<PartialUser>(req)
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

    HTTP.request::<PartialUser>(req)
}

pub fn username_history(id: u64) -> Result<Vec<String>, String> {
    let req = HttpRequest {
        method: Method::GET,
        url: format!("{}/v1/users/{}/username-history", ENDPOINTS.users, id),
        headers: None,
        body: None,
    };

    match HTTP.request::<Data<Username>>(req) {
        Ok(res) => Ok(res.data.into_iter().map(|u| u.name).collect()),
        Err(e) => Err(e),
    }
}

pub fn search(query: &str) -> Result<Vec<UserSearchResult>, String> {
    let req = HttpRequest {
        method: Method::GET,
        url: format!(
            "{}/v1/users/search?keyword={}&limit=100",
            ENDPOINTS.users, query
        ),
        headers: None,
        body: None,
    };

    match HTTP.request::<Data<UserSearchResult>>(req) {
        Ok(res) => Ok(res.data),
        Err(e) => Err(e),
    }
}

pub trait UserMethods {
    fn id(&self) -> u64;
    fn username_history(&self) -> Vec<String> {
        username_history(self.id()).unwrap()
    }
    fn fetch(&self) -> Result<User, String> {
        fetch(self.id())
    }
}

impl UserMethods for User {
    fn id(&self) -> u64 {
        self.id
    }
}

impl UserMethods for PartialUser {
    fn id(&self) -> u64 {
        self.id
    }
}

impl UserMethods for AuthenticatedUser {
    fn id(&self) -> u64 {
        self.id
    }
}

impl UserMethods for UserSearchResult {
    fn id(&self) -> u64 {
        self.id
    }
}
