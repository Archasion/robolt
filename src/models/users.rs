use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::{DataResponse, ENDPOINTS};
use crate::Robolt;
use crate::utilities::client::HttpRequest;

impl Robolt {
    pub fn fetch_user(&self, id: u64) -> Result<User, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/v1/users/{}", ENDPOINTS.users, id),
            body: None,
        };

        self.request::<(), User>(req)
    }

    pub fn authenticated_user(&self) -> Result<PartialUser, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/v1/users/authenticated", ENDPOINTS.users),
            body: None,
        };

        self.request::<(), PartialUser>(req)
    }

    pub fn partial_user(&self, id: u64) -> Result<PartialUser, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/v1/users/{}", ENDPOINTS.users, id),
            body: None,
        };

        self.request::<(), PartialUser>(req)
    }

    pub fn user_id(&self, username: &str) -> Result<u64, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!(
                "{}/users/get-by-username?username={}",
                ENDPOINTS.base, username
            ),
            body: None,
        };

        self.request::<(), UserId>(req).map(|res| res.id)
    }

    pub fn search_users(&self, keyword: &str, limit: u8) -> Result<Vec<PartialUser>, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!(
                "{}/v1/users/search?keyword={}&limit={}",
                ENDPOINTS.users, keyword, limit
            ),
            body: None,
        };

        self.request::<(), DataResponse<PartialUser>>(req)
            .map(|res| res.data)
    }

    pub fn fetch_users(
        &self,
        ids: Vec<u64>,
        exclude_banned: bool,
    ) -> Result<Vec<PartialUser>, String> {
        let post = FetchMany {
            user_ids: ids,
            exclude_banned_users: exclude_banned,
        };

        let req = HttpRequest {
            method: Method::POST,
            endpoint: format!("{}/v1/users", ENDPOINTS.users),
            body: Some(&post),
        };

        self.request::<FetchMany, DataResponse<PartialUser>>(req)
            .map(|res| res.data)
    }

    pub fn find_users(
        &self,
        usernames: Vec<&str>,
        exclude_banned: bool,
    ) -> Result<Vec<PartialUser>, String> {
        let post = FindMany {
            exclude_banned_users: exclude_banned,
            usernames,
        };

        let req = HttpRequest {
            method: Method::POST,
            endpoint: format!("{}/v1/usernames/users", ENDPOINTS.users),
            body: Some(&post),
        };

        self.request::<FindMany, DataResponse<PartialUser>>(req)
            .map(|res| res.data)
    }

    pub fn username_history(&self, id: u64) -> Result<Vec<String>, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/v1/users/{}/username-history", ENDPOINTS.users, id),
            body: None,
        };

        self.request::<(), DataResponse<String>>(req)
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
#[serde(rename_all = "camelCase")]
pub struct PartialUser {
    #[serde(rename = "name")]
    pub username: String,
    pub display_name: String,
    pub id: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FetchMany {
    exclude_banned_users: bool,
    user_ids: Vec<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FindMany<'a> {
    exclude_banned_users: bool,
    usernames: Vec<&'a str>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UserId {
    id: u64,
}