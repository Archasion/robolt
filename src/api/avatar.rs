use std::collections::HashMap;

use reqwest::Method;
use serde::Deserialize;

use crate::api::{Limit, ENDPOINTS};
use crate::errors::RoboltError;
use crate::utilities::client::{Authenticated, EmptyResponse};
use crate::Robolt;

impl Robolt<Authenticated> {
	pub fn my_avatar(&self) -> Result<Avatar, RoboltError> {
		self.request_builder(format!("{}/v1/avatar", ENDPOINTS.avatar))
			.send()
	}

	pub fn add_wearing_asset(&self, asset_id: u64) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/avatar/assets/{}/wear",
			ENDPOINTS.avatar, asset_id
		))
		.method(Method::POST)
		.send::<EmptyResponse>()?;

		Ok(())
	}

	pub fn set_wearing_assets(&self, asset_ids: Vec<u64>) -> Result<InvalidAssets, RoboltError> {
		let mut body = HashMap::new();
		body.insert("assetIds", asset_ids);

		self.request_builder(format!("{}/v1/avatar/set-wearing-assets", ENDPOINTS.avatar))
			.method(Method::POST)
			.send_body(body)
	}

	pub fn remove_wearing_asset(&self, asset_id: u64) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/avatar/assets/{}/remove",
			ENDPOINTS.avatar, asset_id
		))
		.method(Method::POST)
		.send::<EmptyResponse>()?;

		Ok(())
	}

	pub fn redraw_avatar_thumbnail(&self) -> Result<(), RoboltError> {
		self.request_builder(format!("{}/v1/avatar/redraw-thumbnail", ENDPOINTS.avatar))
			.method(Method::POST)
			.send::<EmptyResponse>()?;

		Ok(())
	}

	pub fn set_avatar_type(&self, avatar_type: AvatarType) -> Result<(), RoboltError> {
		let mut body = HashMap::new();
		body.insert("playerAvatarType", avatar_type as u8);

		self.request_builder(format!(
			"{}/v1/avatar/set-player-avatar-type",
			ENDPOINTS.avatar
		))
		.method(Method::POST)
		.send_body::<_, EmptyResponse>(body)?;

		Ok(())
	}
}

impl<State> Robolt<State> {
	pub fn fetch_avatar(&self, user_id: u64) -> Result<Avatar, RoboltError> {
		self.request_builder(format!("{}/v1/users/{}/avatar", ENDPOINTS.avatar, user_id))
			.send()
	}

	pub fn fetch_currently_wearing(&self, user_id: u64) -> Result<Vec<u64>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/currently-wearing",
			ENDPOINTS.avatar, user_id
		))
		.send::<AssetIdsResponse>()
		.map(|res| res.asset_ids)
	}

	pub fn fetch_outfits(&self, user_id: u64) -> OutfitsFilterBuilder<State> {
		OutfitsFilterBuilder::new(user_id, self)
	}
}

impl<'a, State> OutfitsFilterBuilder<'a, State> {
	pub fn new(user_id: u64, client: &'a Robolt<State>) -> Self {
		Self {
			user_id,
			page: 1,
			items_per_page: Limit::default(),
			is_editable: true,
			client,
		}
	}

	pub fn page(mut self, page: u8) -> Self {
		self.page = page;
		self
	}

	pub fn items_per_page(mut self, items_per_page: Limit) -> Self {
		self.items_per_page = items_per_page;
		self
	}

	pub fn editable(mut self, is_editable: bool) -> Self {
		self.is_editable = is_editable;
		self
	}

	pub fn send(self) -> Result<OutfitsResponse, RoboltError> {
		self.client
			.request_builder(format!(
				"{}/v1/users/{}/outfits?page={}&itemsPerPage={}&isEditable={}",
				ENDPOINTS.avatar,
				self.user_id,
				self.page,
				self.items_per_page as u8,
				self.is_editable
			))
			.send()
	}
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvalidAssets {
	pub invalid_assets: Vec<AvatarAsset>,
	pub invalid_asset_ids: Vec<u64>,
	pub success: bool,
}

pub struct OutfitsFilterBuilder<'a, State> {
	user_id: u64,
	page: u8,
	items_per_page: Limit,
	is_editable: bool,
	client: &'a Robolt<State>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutfitsResponse {
	pub filtered_count: u32,
	pub data: Vec<Outfit>,
	pub total: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outfit {
	pub id: u64,
	pub name: String,
	pub is_editable: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssetIdsResponse {
	asset_ids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
	pub player_avatar_type: AvatarType,
	pub default_shirt_applied: bool,
	pub default_pants_applied: bool,
	pub scales: AvatarScales,
	pub body_colors: BodyColors,
	pub assets: Vec<AvatarAsset>,
	pub emotes: Vec<AvatarEmotes>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarScales {
	pub head: f32,
	pub depth: f32,
	pub height: f32,
	pub width: f32,
	pub proportion: f32,
	pub body_type: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyColors {
	pub head_color_id: u32,
	pub torso_color_id: u32,
	pub right_arm_color_id: u32,
	pub left_arm_color_id: u32,
	pub right_leg_color_id: u32,
	pub left_leg_color_id: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarEmotes {
	#[serde(rename = "assetName")]
	pub name: String,
	#[serde(rename = "assetId")]
	pub id: u64,
	pub position: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarAsset {
	pub id: u64,
	pub name: String,
	pub asset_type: AvatarAssetType,
	pub meta: Option<AvatarAssetMeta>,
	pub current_version_id: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AvatarAssetType {
	pub id: u64,
	pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AvatarAssetMeta {
	pub order: u32,
	pub puffiness: Option<f32>,
	pub version: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum AvatarType {
	R6 = 1,
	R15 = 3,
}