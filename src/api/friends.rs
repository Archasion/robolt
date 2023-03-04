use std::io::Error;

use reqwest::Method;
use serde::Deserialize;

use crate::api::{DataResponse, ENDPOINTS};
use crate::api::users::{PartialUser, User};
use crate::Robolt;
use crate::utilities::client::{Authenticated, EmptyResponse};

impl<State> Robolt<State> {
    pub fn count_followers(&self, user_id: u64) -> Result<u64, Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/followers/count",
            ENDPOINTS.friends, user_id
        ))
            .function("count_followers")
            .send::<CountResponse>()
            .map(|res| res.count)
    }

    pub fn count_followings(&self, user_id: u64) -> Result<u64, Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/followings/count",
            ENDPOINTS.friends, user_id
        ))
            .function("count_followings")
            .send::<CountResponse>()
            .map(|res| res.count)
    }

    pub fn count_friends(&self, user_id: u64) -> Result<u64, Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/friends/count",
            ENDPOINTS.friends, user_id
        ))
            .function("count_friends")
            .send::<CountResponse>()
            .map(|res| res.count)
    }

    pub fn fetch_friends(&self, user_id: u64) -> Result<Vec<User>, Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/friends",
            ENDPOINTS.friends, user_id
        ))
            .function("fetch_friends")
            .send::<DataResponse<User>>()
            .map(|res| res.data)
    }

    pub fn fetch_followers(&self, user_id: u64, limit: u8) -> Result<Vec<User>, Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/followers?limit={}",
            ENDPOINTS.friends, user_id, limit
        ))
            .function("fetch_followers")
            .send::<DataResponse<User>>()
            .map(|res| res.data)
    }

    pub fn fetch_followings(&self, user_id: u64, limit: u8) -> Result<Vec<User>, Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/followings?limit={}",
            ENDPOINTS.friends, user_id, limit
        ))
            .function("fetch_followings")
            .send::<DataResponse<User>>()
            .map(|res| res.data)
    }
}

impl Robolt<Authenticated> {
    pub fn my_friend_requests(&self, limit: u8) -> Result<Vec<FriendRequest>, Error> {
        self.request_builder(format!(
            "{}/v1/my/friends/requests?limit={}",
            ENDPOINTS.friends, limit
        ))
            .function("my_friend_requests")
            .send::<DataResponse<FriendRequest>>()
            .map(|res| res.data)
    }

    pub fn my_friend_request_count(&self) -> Result<u64, Error> {
        self.request_builder(format!(
            "{}/v1/user/friend-requests/count",
            ENDPOINTS.friends
        ))
            .function("my_friend_request_count")
            .send::<CountResponse>()
            .map(|res| res.count)
    }

    pub fn my_friend_count(&self) -> Result<u64, Error> {
        self.request_builder(format!("{}/v1/my/friends/count", ENDPOINTS.friends))
            .function("my_friend_count")
            .send::<CountResponse>()
            .map(|res| res.count)
    }

    pub fn unfriend(&self, user_id: u64) -> Result<(), Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/unfriend",
            ENDPOINTS.friends, user_id
        ))
            .function("unfriend")
            .method(Method::POST)
            .send::<EmptyResponse>()?;

        Ok(())
    }

    pub fn unfollow(&self, user_id: u64) -> Result<(), Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/unfollow",
            ENDPOINTS.friends, user_id
        ))
            .function("unfollow")
            .method(Method::POST)
            .send::<EmptyResponse>()?;

        Ok(())
    }

    pub fn decline_friend_request(&self, user_id: u64) -> Result<(), Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/decline-friend-request",
            ENDPOINTS.friends, user_id
        ))
            .function("decline_friend_request")
            .method(Method::POST)
            .send::<EmptyResponse>()?;

        Ok(())
    }

    pub fn accept_friend_request(&self, user_id: u64) -> Result<(), Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/accept-friend-request",
            ENDPOINTS.friends, user_id
        ))
            .function("accept_friend_request")
            .method(Method::POST)
            .send::<EmptyResponse>()?;

        Ok(())
    }

    pub fn decline_all_friend_requests(&self) -> Result<(), Error> {
        self.request_builder(format!(
            "{}/v1/user/friend-requests/decline-all",
            ENDPOINTS.friends
        ))
            .function("decline_all_friend_requests")
            .method(Method::POST)
            .send::<EmptyResponse>()?;

        Ok(())
    }

    pub fn my_online_friends(&self) -> Result<Vec<OnlineFriend>, Error> {
        let user_id = self.fetch_current_user()?.id;

        self.request_builder(format!(
            "{}/v1/users/{}/friends/online",
            ENDPOINTS.friends, user_id
        ))
            .function("my_online_friends")
            .method(Method::GET)
            .send::<DataResponse<OnlineFriend>>()
            .map(|res| res.data)
    }

    pub fn my_friendship_statuses(&self, user_ids: Vec<u64>) -> Result<Vec<Friendship>, Error> {
        let user_id = self.fetch_current_user()?.id;
        let user_ids = user_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        self.request_builder(format!(
            "{}/v1/users/{}/friends/statuses?userIds={}",
            ENDPOINTS.friends, user_id, user_ids
        ))
            .function("my_friendship_statuses")
            .method(Method::GET)
            .send::<DataResponse<Friendship>>()
            .map(|res| res.data)
    }
}

impl Friendship {
    pub fn is_friend(&self) -> bool {
        self.status == FriendshipStatus::Friends
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum FriendshipStatus {
    NotFriends,
    Friends,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Friendship {
    pub id: u64,
    pub status: FriendshipStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPresence {
    #[serde(rename = "UserPresenceType")]
    pub user_presence_type: String,
    #[serde(rename = "UserLocationType")]
    pub user_location_type: String,
    pub last_location: String,
    pub last_online: String,
    pub place_id: Option<u64>,
    pub root_place_id: Option<u64>,
    pub game_instance_id: Option<String>,
    pub universe_id: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OnlineFriend {
    #[serde(flatten)]
    pub user: PartialUser,
    #[serde(rename = "userPresence")]
    pub presence: UserPresence,
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