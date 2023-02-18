use serde::Deserialize;

use crate::models::{DataResponse, ENDPOINTS};
use crate::models::users::User;
use crate::Robolt;

impl Robolt {
    pub fn count_followers(&self, user_id: u64) -> Result<u64, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/followers/count",
            ENDPOINTS.friends, user_id
        ))
            .send::<CountResponse>()
            .map(|res| res.count)
    }

    pub fn count_followings(&self, user_id: u64) -> Result<u64, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/followings/count",
            ENDPOINTS.friends, user_id
        ))
            .send::<CountResponse>()
            .map(|res| res.count)
    }

    pub fn count_friends(&self, user_id: u64) -> Result<u64, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/friends/count",
            ENDPOINTS.friends, user_id
        ))
            .send::<CountResponse>()
            .map(|res| res.count)
    }

    pub fn fetch_friends(&self, user_id: u64) -> Result<Vec<User>, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/friends",
            ENDPOINTS.friends, user_id
        ))
            .send::<DataResponse<User>>()
            .map(|res| res.data)
    }

    pub fn fetch_followers(&self, user_id: u64, limit: u8) -> Result<Vec<User>, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/followers?limit={}",
            ENDPOINTS.friends, user_id, limit
        ))
            .send::<DataResponse<User>>()
            .map(|res| res.data)
    }

    pub fn fetch_followings(&self, user_id: u64, limit: u8) -> Result<Vec<User>, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/followings?limit={}",
            ENDPOINTS.friends, user_id, limit
        ))
            .send::<DataResponse<User>>()
            .map(|res| res.data)
    }

    pub fn my_friend_requests(&self, limit: u8) -> Result<Vec<FriendRequest>, String> {
        self.request_builder(format!(
            "{}/v1/my/friends/requests?limit={}",
            ENDPOINTS.friends, limit
        ))
            .send::<DataResponse<FriendRequest>>()
            .map(|res| res.data)
    }

    pub fn my_friend_count(&self) -> Result<u64, String> {
        self.request_builder(format!("{}/v1/my/friends/count", ENDPOINTS.friends))
            .send::<CountResponse>()
            .map(|res| res.count)
    }
}

#[derive(Deserialize)]
struct CountResponse {
    count: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequest {
    pub friend_request: FriendRequestInfo,
    pub mutual_friends_list: Vec<String>,
    #[serde(flatten)]
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequestInfo {
    pub sent_at: String,
    pub sender_id: u64,
    pub source_universe_id: Option<u64>,
    pub origin_source_type: String,
    pub contact_name: Option<String>,
}