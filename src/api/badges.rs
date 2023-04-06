use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::{DataResponse, ENDPOINTS};
use crate::errors::RoboltError;
use crate::utilities::client::Authenticated;
use crate::Robolt;

impl<State> Robolt<State> {
	pub fn fetch_badge(&self, badge_id: u64) -> Result<Badge, RoboltError> {
		self.request_builder(format!("{}/v1/badges/{}", ENDPOINTS.badges, badge_id))
			.send()
	}

	pub fn fetch_universe_badges(&self, universe_id: u64) -> Result<Vec<Badge>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/universes/{}/badges?limit=100",
			ENDPOINTS.badges, universe_id
		))
		.send::<DataResponse<Badge>>()
		.map(|res| res.data)
	}

	pub fn fetch_user_badges(&self, user_id: u64) -> Result<Vec<Badge>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/badges?limit=100",
			ENDPOINTS.badges, user_id
		))
		.send::<DataResponse<Badge>>()
		.map(|res| res.data)
	}

	pub fn fetch_awarded_timestamps(
		&self,
		user_id: u64,
		badge_ids: Vec<u64>,
	) -> Result<Vec<AwardedBadgeTimestamp>, RoboltError> {
		let badge_ids = badge_ids
			.iter()
			.map(|id| id.to_string())
			.collect::<Vec<String>>()
			.join(",");

		self.request_builder(format!(
			"{}/v1/users/{}/badges/awarded-dates?badgeIds={}",
			ENDPOINTS.badges, user_id, badge_ids
		))
		.send::<DataResponse<AwardedBadgeTimestamp>>()
		.map(|res| res.data)
	}
}

impl Robolt<Authenticated> {
	pub fn update_badge(&self, badge_id: u64) -> BadgeUpdateBuilder {
		BadgeUpdateBuilder::new(badge_id, self)
	}

	pub fn remove_badge(&self, badge_id: u64) -> Result<(), RoboltError> {
		self.request_builder(format!("{}/v1/user/badges/{}", ENDPOINTS.badges, badge_id))
			.method(Method::DELETE)
			.send()
	}
}

impl<'a> BadgeUpdateBuilder<'a> {
	fn new(id: u64, client: &'a Robolt<Authenticated>) -> Self {
		Self {
			id,
			client,
			name: None,
			description: None,
			enabled: None,
		}
	}

	pub fn name(mut self, name: &str) -> Self {
		self.name = Some(name.to_string());
		self
	}

	pub fn description(mut self, description: &str) -> Self {
		self.description = Some(description.to_string());
		self
	}

	pub fn enabled(mut self, enabled: bool) -> Self {
		self.enabled = Some(enabled);
		self
	}

	pub fn update(self) -> Result<(), RoboltError> {
		self.client
			.request_builder(format!("{}/v1/badges/{}", ENDPOINTS.badges, self.id))
			.method(Method::PATCH)
			.send_body(self)
	}
}

#[derive(Serialize)]
pub struct BadgeUpdateBuilder<'a> {
	#[serde(skip_serializing)]
	id: u64,
	#[serde(skip_serializing)]
	client: &'a Robolt<Authenticated>,
	#[serde(skip_serializing_if = "Option::is_none")]
	name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwardedBadgeTimestamp {
	pub badge_id: u64,
	pub awarded_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
	pub id: u64,
	pub name: String,
	pub description: Option<String>,
	pub display_name: String,
	pub display_description: Option<String>,
	pub enabled: bool,
	pub icon_image_id: u64,
	pub display_icon_image_id: u64,
	pub created: String,
	pub updated: String,
	pub statistics: BadgeStats,
	pub awarding_universe: Option<AwardingUniverse>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeStats {
	pub past_day_awarded_count: u64,
	pub awarded_count: u64,
	pub win_rate_percentage: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwardingUniverse {
	pub id: u64,
	pub name: String,
	pub root_place_id: u64,
}