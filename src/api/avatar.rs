use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

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
		let body = HashMap::from([("assetIds", asset_ids)]);

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
		let body = HashMap::from([("avatarType", avatar_type as u8)]);

		self.request_builder(format!(
			"{}/v1/avatar/set-player-avatar-type",
			ENDPOINTS.avatar
		))
		.method(Method::POST)
		.send_body::<_, EmptyResponse>(body)?;

		Ok(())
	}

	pub fn set_body_colors(&self, body_colors: BodyColors) -> Result<(), RoboltError> {
		self.request_builder(format!("{}/v1/avatar/set-body-colors", ENDPOINTS.avatar))
			.method(Method::POST)
			.send_body::<_, EmptyResponse>(body_colors)?;

		Ok(())
	}

	pub fn set_scales(&self, scales: AvatarScales) -> Result<(), RoboltError> {
		self.request_builder(format!("{}/v1/avatar/set-scales", ENDPOINTS.avatar))
			.method(Method::POST)
			.send_body::<_, EmptyResponse>(scales)?;

		Ok(())
	}
}

impl<State> Robolt<State> {
	pub fn fetch_avatar_metadata(&self) -> Result<AvatarMetadata, RoboltError> {
		self.request_builder(format!("{}/v1/avatar/metadata", ENDPOINTS.avatar))
			.send()
	}

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

	pub fn fetch_game_start_info(
		&self,
		universe_id: u64,
	) -> Result<GameStartAvatarInfo, RoboltError> {
		self.request_builder(format!(
			"{}/v1/game-start-info?universeId={universe_id}",
			ENDPOINTS.avatar
		))
		.send()
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssetIdsResponse {
	asset_ids: Vec<u64>,
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarScales {
	pub head: f32,
	pub depth: f32,
	pub height: f32,
	pub width: f32,
	pub proportion: f32,
	pub body_type: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyColors {
	pub head_color_id: u16,
	pub torso_color_id: u16,
	pub right_arm_color_id: u16,
	pub left_arm_color_id: u16,
	pub right_leg_color_id: u16,
	pub left_leg_color_id: u16,
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarMetadata {
	pub enable_default_clothing_message: bool,
	pub is_avatar_scale_embedded_in_tab: bool,
	pub is_body_type_scale_out_of_tab: bool,
	pub scale_height_increment: f32,
	pub scale_width_increment: f32,
	pub scale_head_increment: f32,
	pub scale_proportion_increment: f32,
	pub scale_body_type_increment: f32,
	pub support_proportion_and_body_type: bool,
	pub show_default_clothing_message_on_page_load: bool,
	pub are_three_dee_thumbs_enabled: bool,
	pub is_avatar_wearing_api_calls_locking_on_frontend_enabled: bool,
	pub is_outfit_handling_on_frontend_enabled: bool,
	pub is_justin_ui_changes_enabled: bool,
	pub is_category_reorg_enabled: bool,
	#[serde(rename = "LCEnabledInEditorAndCatalog")]
	pub lcenabled_in_editor_and_catalog: bool,
	#[serde(rename = "isLCCompletelyEnabled")]
	pub is_lccompletely_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStartAvatarInfo {
	pub game_avatar_type: String,
	pub allow_custom_animations: String,
	pub universe_avatar_collision_type: String,
	pub universe_avatar_body_type: String,
	pub joint_positioning_type: String,
	pub message: String,
	pub universe_avatar_min_scales: UniverseAvatarMinScales,
	pub universe_avatar_max_scales: UniverseAvatarMaxScales,
	pub universe_avatar_asset_overrides: Vec<UniverseAvatarAssetOverride>,
	pub moderation_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniverseAvatarMinScales {
	pub height: f32,
	pub width: f32,
	pub head: f32,
	pub depth: f32,
	pub proportion: f32,
	pub body_type: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniverseAvatarMaxScales {
	pub height: f32,
	pub width: f32,
	pub head: f32,
	pub depth: f32,
	pub proportion: f32,
	pub body_type: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniverseAvatarAssetOverride {
	#[serde(rename = "assetID")]
	pub asset_id: f32,
	#[serde(rename = "assetTypeID")]
	pub asset_type_id: f32,
	pub is_player_choice: bool,
}