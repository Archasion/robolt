use reqwest::Method;
use serde::Deserialize;

use crate::api::presence::UserPresence;
use crate::api::routes::RobloxApi;
use crate::api::users::{PartialUser, User};
use crate::api::Limit;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::utils::response::{CountResponse, DataResponse, EmptyResponse};
use crate::Robolt;

impl<State> Robolt<State> {
	pub async fn follower_count(&self, user_id: u64) -> Result<u64, RoboltError> {
		self.request(RobloxApi::Friends, format!("/v1/users/{user_id}/followers/count"))
			.send::<CountResponse<u64>>()
			.await
			.map(|res| res.count)
	}

	pub async fn following_count(&self, user_id: u64) -> Result<u64, RoboltError> {
		self.request(RobloxApi::Friends, format!("/v1/users/{user_id}/followings/count"))
			.send::<CountResponse<u64>>()
			.await
			.map(|res| res.count)
	}

	pub async fn friend_count(&self, user_id: u64) -> Result<u64, RoboltError> {
		self.request(RobloxApi::Friends, format!("/v1/users/{user_id}/friends/count"))
			.send::<CountResponse<u64>>()
			.await
			.map(|res| res.count)
	}

	pub async fn friends(&self, user_id: u64) -> Result<Vec<User>, RoboltError> {
		self.request(RobloxApi::Friends, format!("/v1/users/{user_id}/friends"))
			.send::<DataResponse<User>>()
			.await
			.map(|res| res.data)
	}

	pub async fn followers(&self, user_id: u64, limit: Limit) -> Result<Vec<User>, RoboltError> {
		self.request(
			RobloxApi::Friends,
			format!("/v1/users/{user_id}/followers?limit={}", limit as u8),
		)
		.send::<DataResponse<User>>()
		.await
		.map(|res| res.data)
	}

	pub async fn followings(&self, user_id: u64, limit: Limit) -> Result<Vec<User>, RoboltError> {
		self.request(
			RobloxApi::Friends,
			format!("/v1/users/{user_id}/followings?limit={}", limit as u8),
		)
		.send::<DataResponse<User>>()
		.await
		.map(|res| res.data)
	}
}

impl Robolt<Authenticated> {
	pub async fn friend_requests(&self, limit: Limit) -> Result<Vec<FriendRequest>, RoboltError> {
		self.request(
			RobloxApi::Friends,
			format!("/v1/my/friends/requests?limit={}", limit as u8),
		)
		.send::<DataResponse<FriendRequest>>()
		.await
		.map(|res| res.data)
	}

	pub async fn friend_request_count(&self) -> Result<u64, RoboltError> {
		self.request(RobloxApi::Friends, "/v1/user/friend-requests/count")
			.send::<CountResponse<u64>>()
			.await
			.map(|res| res.count)
	}

	pub async fn friend_count_auth(&self) -> Result<u64, RoboltError> {
		self.request(RobloxApi::Friends, "/v1/my/friends/count")
			.send::<CountResponse<u64>>()
			.await
			.map(|res| res.count)
	}

	pub async fn unfriend(&self, user_id: u64) -> Result<(), RoboltError> {
		self.request(RobloxApi::Friends, format!("/v1/users/{user_id}/unfriend"))
			.method(Method::POST)
			.send::<EmptyResponse>()
			.await?;

		Ok(())
	}

	pub async fn unfollow(&self, user_id: u64) -> Result<(), RoboltError> {
		self.request(RobloxApi::Friends, format!("/v1/users/{user_id}/unfollow"))
			.method(Method::POST)
			.send::<EmptyResponse>()
			.await?;

		Ok(())
	}

	pub async fn decline_friend_request(&self, user_id: u64) -> Result<(), RoboltError> {
		self.request(
			RobloxApi::Friends,
			format!("/v1/users/{user_id}/decline-friend-request"),
		)
		.method(Method::POST)
		.send::<EmptyResponse>()
		.await?;

		Ok(())
	}

	pub async fn accept_friend_request(&self, user_id: u64) -> Result<(), RoboltError> {
		self.request(RobloxApi::Friends, format!("/v1/users/{user_id}/accept-friend-request"))
			.method(Method::POST)
			.send::<EmptyResponse>()
			.await?;

		Ok(())
	}

	pub async fn decline_all_friend_requests(&self) -> Result<(), RoboltError> {
		self.request(RobloxApi::Friends, "/v1/user/friend-requests/decline-all")
			.method(Method::POST)
			.send::<EmptyResponse>()
			.await?;

		Ok(())
	}

	pub async fn online_friends(&self) -> Result<Vec<OnlineFriend>, RoboltError> {
		let user_id = self.me().await?.id;

		self.request(RobloxApi::Friends, format!("/v1/users/{user_id}/friends/online"))
			.method(Method::GET)
			.send::<DataResponse<OnlineFriend>>()
			.await
			.map(|res| res.data)
	}

	pub async fn friendship_statuses(&self, user_ids: Vec<u64>) -> Result<Vec<UserRelationship>, RoboltError> {
		let user_id = self.me().await?.id;
		let user_ids = user_ids
			.iter()
			.map(|id| id.to_string())
			.collect::<Vec<String>>()
			.join(",");

		self.request(
			RobloxApi::Friends,
			format!("/v1/users/{user_id}/friends/statuses?userIds={user_ids}"),
		)
		.method(Method::GET)
		.send::<DataResponse<UserRelationship>>()
		.await
		.map(|res| res.data)
	}
}

impl UserRelationship {
	pub async fn is_friend(&self) -> bool {
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
pub struct UserRelationship {
	pub id: u64,
	pub status: FriendshipStatus,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OnlineFriend {
	#[serde(rename = "userPresence")]
	pub presence: UserPresence,
	#[serde(flatten)]
	pub user: PartialUser,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequest {
	#[serde(flatten)]
	pub user: User,
	pub friend_request: FriendRequestInfo,
	pub mutual_friends_list: Vec<String>,
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