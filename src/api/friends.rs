use reqwest::Method;
use serde::Deserialize;

use crate::api::presence::Presence;
use crate::api::users::{PartialUser, User};
use crate::api::{CountResponse, DataResponse, ENDPOINTS};
use crate::errors::RoboltError;
use crate::utils::client::{Authenticated, EmptyResponse};
use crate::Robolt;

impl<State> Robolt<State> {
	pub fn count_followers(&self, user_id: u64) -> Result<u64, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/followers/count",
			ENDPOINTS.friends, user_id
		))
		.send::<CountResponse<u64>>()
		.map(|res| res.count)
	}

	pub fn count_followings(&self, user_id: u64) -> Result<u64, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/followings/count",
			ENDPOINTS.friends, user_id
		))
		.send::<CountResponse<u64>>()
		.map(|res| res.count)
	}

	pub fn count_friends(&self, user_id: u64) -> Result<u64, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/friends/count",
			ENDPOINTS.friends, user_id
		))
		.send::<CountResponse<u64>>()
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
		.send::<CountResponse<u64>>()
		.map(|res| res.count)
	}

	pub fn my_friend_count(&self) -> Result<u64, RoboltError> {
		self.request_builder(format!("{}/v1/my/friends/count", ENDPOINTS.friends))
			.send::<CountResponse<u64>>()
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
	) -> Result<Vec<UserRelation>, RoboltError> {
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
		.send::<DataResponse<UserRelation>>()
		.map(|res| res.data)
	}
}

impl UserRelation {
	pub fn is_friend(&self) -> bool {
		self.status == FriendshipStatus::Friends
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum FriendshipStatus {
	NotFriends,
	Friends,
	RequestSent,
	RequestReceived,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UserRelation {
	pub id: u64,
	pub status: FriendshipStatus,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OnlineFriend {
	#[serde(rename = "userPresence")]
	pub presence: Presence,
	#[serde(flatten)]
	pub user: PartialUser,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequest {
	#[serde(flatten)]
	pub user: User,
	pub friend_request: FriendRequestDetails,
	pub mutual_friends_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequestDetails {
	pub sent_at: String,
	pub sender_id: u64,
	pub source_universe_id: Option<u64>,
	pub origin_source_type: String,
	pub contact_name: Option<String>,
}