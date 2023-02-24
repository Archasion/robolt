use reqwest::Method;
use serde::Deserialize;

use crate::models::{DataResponse, ENDPOINTS};
use crate::models::users::{PartialUser, User};
use crate::Robolt;
use crate::utilities::client::{Authenticated, EmptyResponse};

impl<State> Robolt<State> {
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
}

impl Robolt<Authenticated> {
    pub fn my_friend_requests(&self, limit: u8) -> Result<Vec<FriendRequest>, String> {
        self.request_builder(format!(
            "{}/v1/my/friends/requests?limit={}",
            ENDPOINTS.friends, limit
        ))
            .send::<DataResponse<FriendRequest>>()
            .map(|res| res.data)
    }

    pub fn my_friend_request_count(&self) -> Result<u64, String> {
        self.request_builder(format!(
            "{}/v1/user/friend-requests/count",
            ENDPOINTS.friends
        ))
            .send::<CountResponse>()
            .map(|res| res.count)
    }

    pub fn my_friend_count(&self) -> Result<u64, String> {
        self.request_builder(format!("{}/v1/my/friends/count", ENDPOINTS.friends))
            .send::<CountResponse>()
            .map(|res| res.count)
    }

    pub fn unfriend(&self, user_id: u64) -> Result<(), String> {
        self.request_builder(format!(
            "{}/v1/users/{}/unfriend",
            ENDPOINTS.friends, user_id
        ))
            .method(Method::POST)
            .send::<EmptyResponse>()?;

        Ok(())
    }

    pub fn unfollow(&self, user_id: u64) -> Result<(), String> {
        self.request_builder(format!(
            "{}/v1/users/{}/unfollow",
            ENDPOINTS.friends, user_id
        ))
            .method(Method::POST)
            .send::<EmptyResponse>()?;

        Ok(())
    }

    pub fn decline_friend_request(&self, user_id: u64) -> Result<(), String> {
        self.request_builder(format!(
            "{}/v1/users/{}/decline-friend-request",
            ENDPOINTS.friends, user_id
        ))
            .method(Method::POST)
            .send::<EmptyResponse>()?;

        Ok(())
    }

    pub fn accept_friend_request(&self, user_id: u64) -> Result<(), String> {
        self.request_builder(format!(
            "{}/v1/users/{}/accept-friend-request",
            ENDPOINTS.friends, user_id
        ))
            .method(Method::POST)
            .send::<EmptyResponse>()?;

        Ok(())
    }

    pub fn decline_all_friend_requests(&self) -> Result<(), String> {
        self.request_builder(format!(
            "{}/v1/user/friend-requests/decline-all",
            ENDPOINTS.friends
        ))
            .method(Method::POST)
            .send::<EmptyResponse>()?;

        Ok(())
    }

    pub fn my_online_friends(&self) -> Result<Vec<OnlineFriend>, String> {
        let user_id = self.fetch_current_user()?.id;

        self.request_builder(format!(
            "{}/v1/users/{}/friends/online",
            ENDPOINTS.friends, user_id
        ))
            .method(Method::GET)
            .send::<DataResponse<OnlineFriend>>()
            .map(|res| res.data)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPresence {
    #[serde(rename = "UserPresenceType")]
    user_presence_type: String,
    #[serde(rename = "UserLocationType")]
    user_location_type: String,
    last_location: String,
    last_online: String,
    place_id: Option<u64>,
    root_place_id: Option<u64>,
    game_instance_id: Option<String>,
    universe_id: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OnlineFriend {
    #[serde(flatten)]
    user: PartialUser,
    #[serde(rename = "userPresence")]
    presence: UserPresence,
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