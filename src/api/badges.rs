use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::routes::RobloxApi;
use crate::api::Limit;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::utils::response::DataResponse;
use crate::Robolt;

impl<State> Robolt<State> {
	pub async fn badge(&self, badge_id: u64) -> Result<Badge, RoboltError> {
		self.request(RobloxApi::Badges, format!("/v1/badges/{badge_id}"))
			.send()
			.await
	}

	pub async fn universe_badges(&self, universe_id: u64, limit: Limit) -> Result<Vec<Badge>, RoboltError> {
		self.request(
			RobloxApi::Badges,
			format!("/v1/universes/{universe_id}/badges?limit={}", limit as u8),
		)
		.send::<DataResponse<Badge>>()
		.await
		.map(|res| res.data)
	}

	pub async fn user_badges(&self, user_id: u64, limit: Limit) -> Result<Vec<Badge>, RoboltError> {
		self.request(
			RobloxApi::Badges,
			format!("/v1/users/{user_id}/badges?limit={}", limit as u8),
		)
		.send::<DataResponse<Badge>>()
		.await
		.map(|res| res.data)
	}

	pub async fn awarded_badge_timestamps(
		&self,
		user_id: u64,
		badge_ids: Vec<u64>,
	) -> Result<Vec<AwardedBadgeTimestamp>, RoboltError> {
		let badge_ids = badge_ids
			.iter()
			.map(|id| id.to_string())
			.collect::<Vec<String>>()
			.join(",");

		self.request(
			RobloxApi::Badges,
			format!("/v1/users/{user_id}/badges/awarded-dates?badgeIds={badge_ids}"),
		)
		.send::<DataResponse<AwardedBadgeTimestamp>>()
		.await
		.map(|res| res.data)
	}
}

impl Robolt<Authenticated> {
	pub fn update_badge(&self, badge_id: u64) -> BadgeUpdateBuilder {
		BadgeUpdateBuilder::new(badge_id, self)
	}

	pub async fn remove_badge(&self, badge_id: u64) -> Result<(), RoboltError> {
		self.request(RobloxApi::Badges, format!("/v1/user/badges/{badge_id}"))
			.method(Method::DELETE)
			.send()
			.await
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

	pub async fn update(self) -> Result<(), RoboltError> {
		self.client
			.request(RobloxApi::Badges, format!("/v1/badges/{}", self.id))
			.method(Method::PATCH)
			.send_body(self)
			.await
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