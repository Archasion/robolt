use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::{DataResponse, EmptyResponse, Limit, ENDPOINTS};
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::Robolt;

impl Robolt<Authenticated> {
	pub async fn avatar_auth(&self) -> Result<Avatar, RoboltError> {
		self.request_builder(format!("{}/v1/avatar", ENDPOINTS.avatar))
			.send()
			.await
	}

	pub async fn add_wearing_asset(&self, asset_id: u64) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/avatar/assets/{}/wear",
			ENDPOINTS.avatar, asset_id
		))
		.method(Method::POST)
		.send::<EmptyResponse>()
		.await?;

		Ok(())
	}

	pub async fn set_wearing_assets(
		&self,
		asset_ids: Vec<u64>,
	) -> Result<InvalidAssets, RoboltError> {
		let body = HashMap::from([("assetIds", asset_ids)]);

		self.request_builder(format!("{}/v1/avatar/set-wearing-assets", ENDPOINTS.avatar))
			.method(Method::POST)
			.send_body(body)
			.await
	}

	pub async fn remove_wearing_asset(&self, asset_id: u64) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/avatar/assets/{}/remove",
			ENDPOINTS.avatar, asset_id
		))
		.method(Method::POST)
		.send::<EmptyResponse>()
		.await?;

		Ok(())
	}

	pub async fn redraw_avatar_thumbnail(&self) -> Result<(), RoboltError> {
		self.request_builder(format!("{}/v1/avatar/redraw-thumbnail", ENDPOINTS.avatar))
			.method(Method::POST)
			.send::<EmptyResponse>()
			.await?;

		Ok(())
	}

	pub async fn set_avatar_type(&self, avatar_type: BodyType) -> Result<(), RoboltError> {
		let body = HashMap::from([("avatarType", avatar_type as u8)]);

		self.request_builder(format!(
			"{}/v1/avatar/set-player-avatar-type",
			ENDPOINTS.avatar
		))
		.method(Method::POST)
		.send_body::<_, EmptyResponse>(body)
		.await?;

		Ok(())
	}

	pub async fn set_body_colors(&self, body_colors: BodyColors) -> Result<(), RoboltError> {
		self.request_builder(format!("{}/v1/avatar/set-body-colors", ENDPOINTS.avatar))
			.method(Method::POST)
			.send_body::<_, EmptyResponse>(body_colors)
			.await?;

		Ok(())
	}

	pub async fn set_scales(&self, scales: BodyScale) -> Result<(), RoboltError> {
		self.request_builder(format!("{}/v1/avatar/set-scales", ENDPOINTS.avatar))
			.method(Method::POST)
			.send_body::<_, EmptyResponse>(scales)
			.await?;

		Ok(())
	}

	pub async fn delete_outfit(&self, outfit_id: u64) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v1/outfits/{}/delete",
			ENDPOINTS.avatar, outfit_id
		))
		.method(Method::POST)
		.send::<EmptyResponse>()
		.await?;

		Ok(())
	}

	pub async fn wear_outfit(&self, outfit_id: u64) -> Result<InvalidAssets, RoboltError> {
		self.request_builder(format!(
			"{}/v1/outfits/{}/wear",
			ENDPOINTS.avatar, outfit_id
		))
		.method(Method::POST)
		.send()
		.await
	}

	pub async fn create_outfit(&self, new_outfit: OutfitV1) -> Result<(), RoboltError> {
		self.request_builder(format!("{}/v1/outfits/create", ENDPOINTS.avatar))
			.method(Method::POST)
			.send_body::<_, EmptyResponse>(new_outfit)
			.await?;

		Ok(())
	}

	pub async fn update_outfit(
		&self,
		outfit_id: u64,
		updated_outfit: OutfitV2,
	) -> Result<(), RoboltError> {
		self.request_builder(format!(
			"{}/v2/outfits/{}/update",
			ENDPOINTS.avatar, outfit_id
		))
		.method(Method::POST)
		.send_body::<_, EmptyResponse>(updated_outfit)
		.await?;

		Ok(())
	}

	pub async fn recent_avatar_items(
		&self,
		item_type: AvatarItemFilter,
	) -> Result<Vec<RecentAvatarItem>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/recent-items/{}/list",
			ENDPOINTS.avatar, item_type as u8
		))
		.send::<DataResponse<RecentAvatarItem>>()
		.await
		.map(|res| res.data)
	}
}

impl<State> Robolt<State> {
	pub async fn avatar_metadata(&self) -> Result<AvatarMetadata, RoboltError> {
		self.request_builder(format!("{}/v1/avatar/metadata", ENDPOINTS.avatar))
			.send()
			.await
	}

	pub async fn avatar(&self, user_id: u64) -> Result<Avatar, RoboltError> {
		self.request_builder(format!("{}/v1/users/{}/avatar", ENDPOINTS.avatar, user_id))
			.send()
			.await
	}

	pub async fn currently_wearing(&self, user_id: u64) -> Result<Vec<u64>, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/currently-wearing",
			ENDPOINTS.avatar, user_id
		))
		.send::<AssetIdsResponse>()
		.await
		.map(|res| res.asset_ids)
	}

	pub fn outfits(&self, user_id: u64) -> OutfitFilterBuilder<State> {
		OutfitFilterBuilder::new(user_id, self)
	}

	pub async fn game_start_info(
		&self,
		universe_id: u64,
	) -> Result<GameStartAvatarInfo, RoboltError> {
		self.request_builder(format!(
			"{}/v1/game-start-info?universeId={universe_id}",
			ENDPOINTS.avatar
		))
		.send()
		.await
	}

	pub async fn outfit(&self, outfit_id: u64) -> Result<DetailedOutfit, RoboltError> {
		self.request_builder(format!(
			"{}/v1/outfits/{}/details",
			ENDPOINTS.avatar, outfit_id
		))
		.send()
		.await
	}
}

impl<'a, State> OutfitFilterBuilder<'a, State> {
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

	pub async fn send(self) -> Result<FilteredOutfitResponse, RoboltError> {
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
			.await
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedOutfit {
	pub id: u64,
	#[serde(rename = "playerAvatarType")]
	pub avatar_type: BodyType,
	pub name: String,
	pub is_editable: bool,
	pub body_colors: BodyColors,
	pub assets: Vec<AvatarAsset>,
	pub scale: BodyScale,
	pub outfit_type: OutfitType,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutfitV1 {
	#[serde(rename = "playerAvatarType")]
	pub avatar_type: BodyType,
	pub name: &'static str,
	pub body_colors: BodyColors,
	pub asset_ids: Vec<u64>,
	pub scale: BodyScale,
	pub outfit_type: OutfitType,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutfitV2 {
	#[serde(rename = "playerAvatarType")]
	pub avatar_type: BodyType,
	pub name: &'static str,
	pub body_colors: BodyColors,
	pub assets: Vec<AvatarAsset>,
	pub scale: BodyScale,
	pub outfit_type: OutfitType,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum OutfitType {
	Invalid,
	Avatar,
	DynamicHead,
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

pub struct OutfitFilterBuilder<'a, State> {
	user_id: u64,
	page: u8,
	items_per_page: Limit,
	is_editable: bool,
	client: &'a Robolt<State>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilteredOutfitResponse {
	pub filtered_count: u32,
	pub data: Vec<OutfitInfo>,
	pub total: u64,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum AvatarItemFilter {
	#[default]
	All = 0,
	Clothing = 1,
	Body = 2,
	Animations = 3,
	Accessories = 4,
	Outfits = 5,
	Gears = 6,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum AvatarItemType {
	Asset,
	Outfit,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutfitInfo {
	pub id: u64,
	pub name: String,
	pub is_editable: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
	pub player_avatar_type: BodyType,
	pub default_shirt_applied: bool,
	pub default_pants_applied: bool,
	pub scales: BodyScale,
	pub body_colors: BodyColors,
	pub assets: Vec<AvatarAsset>,
	pub emotes: Vec<AvatarEmotes>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyScale {
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
	pub head_color_id: BrickColor,
	pub torso_color_id: BrickColor,
	pub right_arm_color_id: BrickColor,
	pub left_arm_color_id: BrickColor,
	pub right_leg_color_id: BrickColor,
	pub left_leg_color_id: BrickColor,
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarAsset {
	pub id: u64,
	pub name: String,
	pub asset_type: AvatarAssetInfo,
	pub meta: Option<AvatarAssetMeta>,
	pub current_version_id: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentAvatarItem {
	pub id: u64,
	pub name: String,
	#[serde(rename = "type")]
	pub item_type: AvatarItemType,
	pub asset_type: AvatarAssetInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AvatarAssetInfo {
	pub id: u64,
	pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AvatarAssetMeta {
	pub order: u32,
	pub puffiness: Option<f32>,
	pub version: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum BodyType {
	#[default]
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
	pub universe_avatar_min_scales: BodyScale,
	pub universe_avatar_max_scales: BodyScale,
	pub universe_avatar_asset_overrides: Vec<UniverseAvatarAssetOverride>,
	pub moderation_status: Option<String>,
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u16)]
pub enum BrickColor {
	#[default]
	White = 1,
	Grey = 2,
	LightYellow = 3,
	BrickYellow = 5,
	LightGreenMint = 6,
	LightReddishViolet = 9,
	PastelBlue = 11,
	LightOrangeBrown = 12,
	Nougat = 18,
	BrightRed = 21,
	MedReddishViolet = 22,
	BrightBlue = 23,
	BrightYellow = 24,
	EarthOrange = 25,
	Black = 26,
	DarkGrey = 27,
	DarkGreen = 28,
	MediumGreen = 29,
	LightYellowishOrange = 36,
	BrightGreen = 37,
	DarkOrange = 38,
	LightBluishViolet = 39,
	Transparent = 40,
	TrRed = 41,
	TrLgBlue = 42,
	TrBlue = 43,
	TrYellow = 44,
	LightBlue = 45,
	TrFluReddishOrange = 47,
	TrGreen = 48,
	TrFluGreen = 49,
	PhosphWhite = 50,
	LightRed = 100,
	MediumRed = 101,
	MediumBlue = 102,
	LightGrey = 103,
	BrightViolet = 104,
	BrYellowishOrange = 105,
	BrightOrange = 106,
	BrightBluishGreen = 107,
	EarthYellow = 108,
	BrightBluishViolet = 110,
	TrBrown = 111,
	MediumBluishViolet = 112,
	TrMediReddishViolet = 113,
	MedYellowishGreen = 115,
	MedBluishGreen = 116,
	LightBluishGreen = 118,
	BrYellowishGreen = 119,
	LigYellowishGreen = 120,
	MedYellowishOrange = 121,
	BrReddishOrange = 123,
	BrightReddishViolet = 124,
	LightOrange = 125,
	TrBrightBluishViolet = 126,
	LightGold = 127,
	DarkNougat = 128,
	Silver = 131,
	NeonOrange = 133,
	NeonGreen = 134,
	SandBlue = 135,
	SandViolet = 136,
	MediumOrange = 137,
	SandYellow = 138,
	EarthBlue = 140,
	EarthGreen = 141,
	TrFluBlue = 143,
	SandBlueMetallic = 145,
	SandVioletMetallic = 146,
	SandYellowMetallic = 147,
	DarkGreyMetallic = 148,
	BlackMetallic = 149,
	LightGreyMetallic = 150,
	SandGreen = 151,
	SandRed = 153,
	DarkRed = 154,
	TrFluYellow = 157,
	TrFluRed = 158,
	GunMetallic = 168,
	RedFlipFlop = 176,
	YellowFlipFlop = 178,
	SilverFlipFlop = 179,
	Curry = 180,
	FireYellow = 190,
	FlameYellowishOrange = 191,
	ReddishBrown = 192,
	FlameReddishOrange = 193,
	MediumStoneGrey = 194,
	RoyalBlue = 195,
	DarkRoyalBlue = 196,
	BrightReddishLilac = 198,
	DarkStoneGrey = 199,
	LemonMetallic = 200,
	LightStoneGrey = 208,
	DarkCurry = 209,
	FadedGreen = 210,
	Turquoise = 211,
	LightRoyalBlue = 212,
	MediumRoyalBlue = 213,
	Rust = 216,
	Brown = 217,
	ReddishLilac = 218,
	Lilac = 219,
	LightLilac = 220,
	BrightPurple = 221,
	LightPurple = 222,
	LightPink = 223,
	LightBrickYellow = 224,
	WarmYellowishOrange = 225,
	CoolYellow = 226,
	DoveBlue = 232,
	MediumLilac = 268,
	SlimeGreen = 301,
	SmokyGrey = 302,
	DarkBlue = 303,
	ParsleyGreen = 304,
	SteelBlue = 305,
	StormBlue = 306,
	Lapis = 307,
	DarkIndigo = 308,
	SeaGreen = 309,
	Shamrock = 310,
	Fossil = 311,
	Mulberry = 312,
	ForestGreen = 313,
	CadetBlue = 314,
	ElectricBlue = 315,
	Eggplant = 316,
	Moss = 317,
	Artichoke = 318,
	SageGreen = 319,
	GhostGrey = 320,
	Lilac2 = 321,
	Plum = 322,
	Olivine = 323,
	LaurelGreen = 324,
	QuillGrey = 325,
	Crimson = 327,
	Mint = 328,
	BabyBlue = 329,
	CarnationPink = 330,
	Persimmon = 331,
	Maroon = 332,
	Gold = 333,
	DaisyOrange = 334,
	Pearl = 335,
	Fog = 336,
	Salmon = 337,
	Terracotta = 338,
	Cocoa = 339,
	Wheat = 340,
	Buttermilk = 341,
	Mauve = 342,
	Sunrise = 343,
	Tawny = 344,
	Rust2 = 345,
	Cashmere = 346,
	Khaki = 347,
	LilyWhite = 348,
	Seashell = 349,
	Burgundy = 350,
	Cork = 351,
	Burlap = 352,
	Beige = 353,
	Oyster = 354,
	PineCone = 355,
	FawnBrown = 356,
	HurricaneGrey = 357,
	CloudyGrey = 358,
	Linen = 359,
	Copper = 360,
	DirtBrown = 361,
	Bronze = 362,
	Flint = 363,
	DarkTaupe = 364,
	BurntSienna = 365,
	InstitutionalWhite = 1001,
	MidGray = 1002,
	ReallyBlack = 1003,
	ReallyRed = 1004,
	DeepOrange = 1005,
	Alder = 1006,
	DustyRose = 1007,
	Olive = 1008,
	NewYeller = 1009,
	ReallyBlue = 1010,
	NavyBlue = 1011,
	DeepBlue = 1012,
	Cyan = 1013,
	CGABrown = 1014,
	Magenta = 1015,
	Pink = 1016,
	DeepOrange2 = 1017,
	Teal = 1018,
	Toothpaste = 1019,
	LimeGreen = 1020,
	Camo = 1021,
	Grime = 1022,
	Lavender = 1023,
	PastelLightBlue = 1024,
	PastelOrange = 1025,
	PastelViolet = 1026,
	PastelBlueGreen = 1027,
	PastelGreen = 1028,
	PastelYellow = 1029,
	PastelBrown = 1030,
	RoyalPurple = 1031,
	HotPink = 1032,
}