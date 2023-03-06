use reqwest::Method;
use serde::Deserialize;

use crate::api::users::{PartialUser, User};
use crate::api::{DataResponse, ENDPOINTS};
use crate::errors::RoboltError;
use crate::utilities::client::{Authenticated, EmptyResponse};
use crate::Robolt;

impl<State> Robolt<State> {
	pub fn count_followers(&self, user_id: u64) -> Result<u64, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/followers/count",
			ENDPOINTS.friends, user_id
		))
		.send::<CountResponse>()
		.map(|res| res.count)
	}

	pub fn count_followings(&self, user_id: u64) -> Result<u64, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/followings/count",
			ENDPOINTS.friends, user_id
		))
		.send::<CountResponse>()
		.map(|res| res.count)
	}

	pub fn count_friends(&self, user_id: u64) -> Result<u64, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/friends/count",
			ENDPOINTS.friends, user_id
		))
		.send::<CountResponse>()
		.map(|res| res.count)
	}

	pub fn fetch_friends(&self, user_id: u64) -> Result<Vec<User>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/friends",
			ENDPOINTS.friends, user_id
		))
		.send::<DataResponse<User>>()
		.map(|res| res.data)
	}

	pub fn fetch_followers(&self, user_id: u64, limit: u8) -> Result<Vec<User>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/followers?limit={}",
			ENDPOINTS.friends, user_id, limit
		))
		.send::<DataResponse<User>>()
		.map(|res| res.data)
	}

	pub fn fetch_followings(&self, user_id: u64, limit: u8) -> Result<Vec<User>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/followings?limit={}",
			ENDPOINTS.friends, user_id, limit
		))
		.send::<DataResponse<User>>()
		.map(|res| res.data)
	}
}

impl Robolt<Authenticated> {
	pub fn my_friend_requests(&self, limit: u8) -> Result<Vec<FriendRequest>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/my/friends/requests?limit={}",
			ENDPOINTS.friends, limit
		))
		.send::<DataResponse<FriendRequest>>()
		.map(|res| res.data)
	}

	pub fn my_friend_request_count(&self) -> Result<u64, RoboltError> {
		self.request_builder(format!(
			"{}/v1/user/friend-requests/count",
			ENDPOINTS.friends
		))
		.send::<CountResponse>()
		.map(|res| res.count)
	}

	pub fn my_friend_count(&self) -> Result<u64, RoboltError> {
		self.request_builder(format!("{}/v1/my/friends/count", ENDPOINTS.friends))
			.send::<CountResponse>()
			.map(|res| res.count)
	}

	pub fn unfriend(&self, user_id: u64) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/unfriend",
			ENDPOINTS.friends, user_id
		))
		.method(Method::POST)
		.send::<EmptyResponse>()?;

		Ok(())
	}

	pub fn unfollow(&self, user_id: u64) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/unfollow",
			ENDPOINTS.friends, user_id
		))
		.method(Method::POST)
		.send::<EmptyResponse>()?;

		Ok(())
	}

	pub fn decline_friend_request(&self, user_id: u64) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/decline-friend-request",
			ENDPOINTS.friends, user_id
		))
		.method(Method::POST)
		.send::<EmptyResponse>()?;

		Ok(())
	}

	pub fn accept_friend_request(&self, user_id: u64) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/accept-friend-request",
			ENDPOINTS.friends, user_id
		))
		.method(Method::POST)
		.send::<EmptyResponse>()?;

		Ok(())
	}

	pub fn decline_all_friend_requests(&self) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/user/friend-requests/decline-all",
			ENDPOINTS.friends
		))
		.method(Method::POST)
		.send::<EmptyResponse>()?;

		Ok(())
	}

	pub fn my_online_friends(&self) -> Result<Vec<OnlineFriend>, RoboltError> {
		let user_id = self.fetch_current_user()?.id;

		self.request_builder(format!(
			"{}/v1/users/{}/friends/online",
			ENDPOINTS.friends, user_id
		))
		.method(Method::GET)
		.send::<DataResponse<OnlineFriend>>()
		.map(|res| res.data)
	}

	pub fn my_friendship_statuses(
		&self,
		user_ids: Vec<u64>,
	) -> Result<Vec<Friendship>, RoboltError> {
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
	pub id:     u64,
	pub status: FriendshipStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPresence {
	#[serde(rename = "UserPresenceType")]
	pub user_presence_type: String,
	#[serde(rename = "UserLocationType")]
	pub user_location_type: String,
	pub last_location:      String,
	pub last_online:        String,
	pub place_id:           Option<u64>,
	pub root_place_id:      Option<u64>,
	pub game_instance_id:   Option<String>,
	pub universe_id:        Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OnlineFriend {
	#[serde(flatten)]
	pub user:     PartialUser,
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
	#[serde(flatten)]
	pub user:                User,
	pub friend_request:      FriendRequestInfo,
	pub mutual_friends_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequestInfo {
	pub sent_at:            String,
	pub sender_id:          u64,
	pub source_universe_id: Option<u64>,
	pub origin_source_type: String,
	pub contact_name:       Option<String>,
}