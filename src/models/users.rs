use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::{DataResponse, ENDPOINTS};
use crate::Robolt;

impl Robolt {
    pub fn fetch_user(&self, id: u64) -> Result<User, String> {
        self.request_builder(format!("{}/v1/users/{}", ENDPOINTS.users, id))
            .send()
    }

    pub fn user(&self) -> Result<PartialUser, String> {
        self.request_builder(format!("{}/v1/users/authenticated", ENDPOINTS.users))
            .send()
    }

    pub fn fetch_partial_user(&self, id: u64) -> Result<PartialUser, String> {
        self.request_builder(format!("{}/v1/users/{}", ENDPOINTS.users, id))
            .send()
    }

    pub fn user_id(&self, username: &str) -> Result<u64, String> {
        self.request_builder(format!(
            "{}/users/get-by-username?username={}",
            ENDPOINTS.base, username
        ))
            .send::<UserId>()
            .map(|res| res.id)
    }

    pub fn search_users(&self, keyword: &str, limit: u8) -> Result<Vec<PartialUser>, String> {
        self.request_builder(format!(
            "{}/v1/users/search?keyword={}&limit={}",
            ENDPOINTS.users, keyword, limit
        ))
            .send::<DataResponse<PartialUser>>()
            .map(|res| res.data)
    }

    pub fn fetch_users(
        &self,
        ids: Vec<u64>,
        exclude_banned: bool,
    ) -> Result<Vec<PartialUser>, String> {
        let post = SearchById {
            user_ids: ids,
            exclude_banned_users: exclude_banned,
        };

        self.request_builder(format!("{}/v1/users", ENDPOINTS.users))
            .method(Method::POST)
            .send_body::<_, DataResponse<PartialUser>>(Some(&post))
            .map(|res| res.data)
    }

    pub fn find_users(
        &self,
        usernames: Vec<&str>,
        exclude_banned: bool,
    ) -> Result<Vec<PartialUser>, String> {
        let post = SearchByUsername {
            exclude_banned_users: exclude_banned,
            usernames,
        };

        self.request_builder(format!("{}/v1/usernames/users", ENDPOINTS.users))
            .method(Method::POST)
            .send_body::<_, DataResponse<PartialUser>>(Some(&post))
            .map(|res| res.data)
    }

    pub fn username_history(&self, id: u64) -> Result<Vec<String>, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/username-history",
            ENDPOINTS.users, id
        ))
            .send::<DataResponse<String>>()
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
struct SearchById {
    exclude_banned_users: bool,
    user_ids: Vec<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchByUsername<'a> {
    exclude_banned_users: bool,
    usernames: Vec<&'a str>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UserId {
    id: u64,
}