use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::{DataResponse, ENDPOINTS};
use crate::utilities::client::{Authenticated, EmptyResponse};
use crate::utilities::errors::RoboltError;
use crate::Robolt;

impl<State> Robolt<State> {
	pub fn fetch_user(&self, user_id: u64) -> Result<User, RoboltError> {
		self.request_builder(format!("{}/v1/users/{}", ENDPOINTS.users, user_id))
			.send()
	}

	pub fn fetch_partial_user(&self, user_id: u64) -> Result<PartialUser, RoboltError> {
		self.request_builder(format!("{}/v1/users/{}", ENDPOINTS.users, user_id))
			.send()
	}

	pub fn fetch_user_id(&self, username: &str) -> Result<u64, RoboltError> {
		self.request_builder(format!(
			"{}/users/get-by-username?username={}",
			ENDPOINTS.base, username
		))
		.send::<UserId>()
		.map(|res| res.id)
	}

	pub fn search_users(&self, keyword: &str, limit: u8) -> Result<Vec<PartialUser>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/search?keyword={}&limit={}",
			ENDPOINTS.users, keyword, limit
		))
		.send::<DataResponse<PartialUser>>()
		.map(|res| res.data)
	}

	pub fn fetch_users(
		&self,
		user_ids: Vec<u64>,
		exclude_banned: bool,
	) -> Result<Vec<PartialUser>, RoboltError> {
		let post = SearchPartialUsersById {
			exclude_banned_users: exclude_banned,
			user_ids,
		};

		self.request_builder(format!("{}/v1/users", ENDPOINTS.users))
			.method(Method::POST)
			.send_body::<_, DataResponse<PartialUser>>(&post)
			.map(|res| res.data)
	}

	pub fn fetch_username_history(&self, user_id: u64) -> Result<Vec<String>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/username-history",
			ENDPOINTS.users, user_id
		))
		.send::<DataResponse<String>>()
		.map(|res| res.data)
	}

	pub fn validate_display_name(
		&self,
		display_name: &str,
		date_of_birth: &str,
	) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/display-names/validate?displayName={}&birthdate={}",
			ENDPOINTS.users, display_name, date_of_birth
		))
		.send::<EmptyResponse>()?;

		Ok(())
	}
}

impl Robolt<Authenticated> {
	pub fn fetch_current_user(&self) -> Result<PartialUser, RoboltError> {
		self.request_builder(format!("{}/v1/users/authenticated", ENDPOINTS.users))
			.send()
	}

	pub fn fetch_users_by_username(
		&self,
		usernames: Vec<&str>,
		exclude_banned: bool,
	) -> Result<Vec<PartialUser>, RoboltError> {
		let post = SearchPartialUsersByUsername {
			exclude_banned_users: exclude_banned,
			usernames,
		};

		self.request_builder(format!("{}/v1/usernames/users", ENDPOINTS.users))
			.method(Method::POST)
			.send_body::<_, DataResponse<PartialUser>>(&post)
			.map(|res| res.data)
	}
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	#[serde(rename = "name")]
	pub username:                  String,
	pub external_app_display_name: Option<String>,
	pub description:               Option<String>,
	pub created:                   String,
	pub is_banned:                 bool,
	pub has_verified_badge:        bool,
	pub id:                        u64,
	pub display_name:              String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialUser {
	#[serde(alias = "userId")]
	pub id:           u64,
	#[serde(rename = "name")]
	pub username:     String,
	pub display_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchPartialUsersById {
	exclude_banned_users: bool,
	user_ids:             Vec<u64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchPartialUsersByUsername<'a> {
	exclude_banned_users: bool,
	usernames:            Vec<&'a str>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UserId {
	id: u64,
}