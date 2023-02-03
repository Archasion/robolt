use std::cell::RefCell;
use std::rc::Rc;

use reqwest::blocking::Client;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::{DataResponse, ENDPOINTS};
use crate::utilities::client::{BorrowClient, HttpRequest};

pub struct UserBuilder {
    pub(crate) client: Rc<RefCell<Client>>,
}

impl UserBuilder {
    pub(crate) fn from(client: Rc<RefCell<Client>>) -> Self {
        Self { client }
    }

    pub fn fetch(&self, id: u64) -> Result<User, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/v1/users/{}", ENDPOINTS.users, id),
            body: None,
        };

        self.client.request::<(), User>(req)
    }

    pub fn me(&self) -> Result<AuthenticatedUser, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/v1/users/authenticated", ENDPOINTS.users),
            body: None,
        };

        self.client.request::<(), AuthenticatedUser>(req)
    }

    pub fn partial(&self, id: u64) -> Result<PartialUser, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/users/{}", ENDPOINTS.base, id),
            body: None,
        };

        self.client.request::<(), PartialUser>(req)
    }

    pub fn find(&self, username: &str) -> Result<PartialUser, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!(
                "{}/users/get-by-username?username={}",
                ENDPOINTS.base, username
            ),
            body: None,
        };

        self.client.request::<(), PartialUser>(req)
    }

    pub fn search(&self, keyword: &str, limit: u8) -> Result<Vec<UserSearchResult>, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!(
                "{}/v1/users/search?keyword={}&limit={}",
                ENDPOINTS.users, keyword, limit
            ),
            body: None,
        };

        self.client
            .request::<(), DataResponse<UserSearchResult>>(req)
            .map(|res| res.data)
    }

    pub fn fetch_many(
        &self,
        ids: Vec<u64>,
        exclude_banned: bool,
    ) -> Result<Vec<UserSearchResult>, String> {
        let post = UserFetchMultiple {
            user_ids: ids,
            exclude_banned_users: exclude_banned,
        };

        let req = HttpRequest {
            method: Method::POST,
            endpoint: format!("{}/v1/users", ENDPOINTS.users),
            body: Some(&post),
        };

        self.client
            .request::<UserFetchMultiple, DataResponse<UserSearchResult>>(req)
            .map(|res| res.data)
    }

    pub fn username_history(&self, id: u64) -> Result<Vec<String>, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/v1/users/{}/username-history", ENDPOINTS.users, id),
            body: None,
        };

        self.client
            .request::<(), DataResponse<String>>(req)
            .map(|res| res.data)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "name")]
    pub username: String,
    pub description: String,
    pub created: String,
    pub is_banned: bool,
    pub external_app_display_name: Option<String>,
    pub has_verified_badge: bool,
    pub id: u64,
    pub display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PartialUser {
    pub username: String,
    pub id: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUser {
    #[serde(rename = "name")]
    pub username: String,
    pub display_name: String,
    pub id: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchResult {
    #[serde(rename = "name")]
    pub username: String,
    pub display_name: String,
    pub has_verified_badge: bool,
    pub id: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct UserFetchMultiple {
    exclude_banned_users: bool,
    user_ids: Vec<u64>,
}