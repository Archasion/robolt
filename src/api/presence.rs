use std::collections::HashMap;

use reqwest::Method;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::api::ENDPOINTS;
use crate::errors::RoboltError;
use crate::Robolt;

impl<State> Robolt<State> {
	pub fn fetch_presences(&self, user_ids: Vec<u64>) -> Result<Vec<Presence>, RoboltError> {
		let mut body = HashMap::new();
		body.insert("userIds", user_ids);

		self.request_builder(format!("{}/v1/presence/users", ENDPOINTS.presence))
			.method(Method::POST)
			.send_body::<_, DetailedPresences>(body)
			.map(|res| res.user_presences)
	}

	pub fn fetch_last_online(
		&self,
		user_ids: Vec<u64>,
	) -> Result<Vec<PartialPresence>, RoboltError> {
		let mut body = HashMap::new();
		body.insert("userIds", user_ids);

		self.request_builder(format!("{}/v1/presence/last-online", ENDPOINTS.presence))
			.method(Method::POST)
			.send_body::<_, Presences>(body)
			.map(|res| res.last_online_timestamps)
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Presence {
	#[serde(alias = "UserPresenceType")]
	pub user_presence_type: UserPresenceType,
	#[serde(rename = "UserLocationType")]
	pub user_location_type: Option<UserLocationType>,
	#[serde(default)]
	pub user_id:            u64,
	pub last_online:        String,
	pub last_location:      String,
	pub game_id:            Option<u64>,
	pub place_id:           Option<u64>,
	pub universe_id:        Option<u64>,
	pub root_place_id:      Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialPresence {
	pub user_id:     u64,
	pub last_online: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum UserPresenceType {
	Offline  = 0,
	Online   = 1,
	InGame   = 2,
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
struct DetailedPresences {
	user_presences: Vec<Presence>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Presences {
	last_online_timestamps: Vec<PartialPresence>,
}