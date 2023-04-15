use std::collections::HashMap;

use reqwest::Method;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::api::ENDPOINTS;
use crate::errors::RoboltError;
use crate::Robolt;

impl<State> Robolt<State> {
	pub async fn fetch_presences(
		&self,
		user_ids: Vec<u64>,
	) -> Result<Vec<UserPresence>, RoboltError> {
		let body = HashMap::from([("userIds", user_ids)]);

		self.request_builder(format!("{}/v1/presence/users", ENDPOINTS.presence))
			.method(Method::POST)
			.send_body::<_, UserPresences>(body)
			.await
			.map(|res| res.user_presences)
	}

	pub async fn fetch_partial_presences(
		&self,
		user_ids: Vec<u64>,
	) -> Result<Vec<PartialUserPresence>, RoboltError> {
		let body = HashMap::from([("userIds", user_ids)]);

		self.request_builder(format!("{}/v1/presence/last-online", ENDPOINTS.presence))
			.method(Method::POST)
			.send_body::<_, LastOnlineTimestamps>(body)
			.await
			.map(|res| res.last_online_timestamps)
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPresence {
	#[serde(alias = "UserPresenceType")]
	pub user_presence_type: UserPresenceType,
	#[serde(rename = "UserLocationType")]
	pub user_location_type: Option<UserLocationType>,
	#[serde(default)]
	pub user_id: u64,
	pub last_online: String,
	pub last_location: String,
	pub game_id: Option<u64>,
	pub place_id: Option<u64>,
	pub universe_id: Option<u64>,
	pub root_place_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialUserPresence {
	pub user_id: u64,
	pub last_online: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum UserPresenceType {
	Offline = 0,
	Online = 1,
	InGame = 2,
	InStudio = 3,
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum UserLocationType {
	Page,
	Game,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserPresences {
	user_presences: Vec<UserPresence>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LastOnlineTimestamps {
	last_online_timestamps: Vec<PartialUserPresence>,
}
