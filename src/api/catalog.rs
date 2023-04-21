use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::Deserialize_repr;

use crate::api::routes::RobloxApi;
use crate::api::Limit;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::utils::response::{DataResponse, EmptyResponse};
use crate::Robolt;

impl<State> Robolt<State> {
	pub async fn asset_parent_bundles(&self, asset_id: u64, limit: Limit) -> Result<Vec<Bundle>, RoboltError> {
		self.request(
			RobloxApi::Catalog,
			format!("/v1/assets/{asset_id}/bundles?limit={}", limit as u8),
		)
		.send::<DataResponse<Bundle>>()
		.await
		.map(|res| res.data)
	}

	pub async fn bundle(&self, bundle_id: u64) -> Result<Bundle, RoboltError> {
		self.request(RobloxApi::Catalog, format!("/v1/bundles/{bundle_id}/details"))
			.send()
			.await
	}

	/// Fetches [Bundle] recommendation list based on a bundle associated with
	/// the provided ID
	///
	/// ### Arguments
	/// * `bundle_id` - The ID of the bundle to base recommendations on
	/// * `amount` - The amount of recommendations to fetch (amount <= 50)
	pub async fn bundle_recommendations(&self, bundle_id: u64, amount: u8) -> Result<Vec<Bundle>, RoboltError> {
		self.request(
			RobloxApi::Catalog,
			format!("/v1/bundles/{bundle_id}/recommendations?numItems={amount}"),
		)
		.send::<DataResponse<Bundle>>()
		.await
		.map(|res| res.data)
	}

	pub async fn bundles(&self, bundle_ids: Vec<u64>) -> Result<Vec<Bundle>, RoboltError> {
		let bundle_ids = bundle_ids
			.iter()
			.map(|id| id.to_string())
			.collect::<Vec<String>>()
			.join(",");

		self.request(
			RobloxApi::Catalog,
			format!("/v1/bundles/details?bundleIds={bundle_ids}"),
		)
		.send()
		.await
	}

	pub async fn user_bundles(&self, user_id: u64) -> Result<Vec<OwnedBundle>, RoboltError> {
		self.request(RobloxApi::Catalog, format!("/v1/users/{user_id}/bundles"))
			.send::<DataResponse<OwnedBundle>>()
			.await
			.map(|res| res.data)
	}

	pub async fn user_bundles_filter(
		&self,
		user_id: u64,
		bundle_type: BundleType,
		limit: Limit,
	) -> Result<Vec<OwnedBundle>, RoboltError> {
		self.request(
			RobloxApi::Catalog,
			format!(
				"/v1/users/{user_id}/bundles/{}?limit={}",
				bundle_type as u8, limit as u8
			),
		)
		.send::<DataResponse<OwnedBundle>>()
		.await
		.map(|res| res.data)
	}

	pub async fn asset_favorite_count(&self, asset_id: u64) -> Result<u64, RoboltError> {
		self.request(RobloxApi::Catalog, format!("/v1/favorites/assets/{asset_id}/count"))
			.send()
			.await
	}

	pub async fn bundle_favorite_count(&self, bundle_id: u64) -> Result<u64, RoboltError> {
		self.request(RobloxApi::Catalog, format!("/v1/favorites/bundles/{bundle_id}/count"))
			.send()
			.await
	}
}

impl Robolt<Authenticated> {
	pub async fn favorite_asset(&self, user_id: u64, asset_id: u64) -> Result<(), RoboltError> {
		self.request(
			RobloxApi::Catalog,
			format!("/v1/favorites/users/{user_id}/assets/{asset_id}/favorite"),
		)
		.method(Method::POST)
		.send::<EmptyResponse>()
		.await?;

		Ok(())
	}

	pub async fn unfavorite_asset(&self, user_id: u64, asset_id: u64) -> Result<(), RoboltError> {
		self.request(
			RobloxApi::Catalog,
			format!("/v1/favorites/users/{user_id}/assets/{asset_id}/favorite"),
		)
		.method(Method::DELETE)
		.send::<EmptyResponse>()
		.await?;

		Ok(())
	}

	pub async fn favorite_bundle(&self, user_id: u64, bundle_id: u64) -> Result<(), RoboltError> {
		self.request(
			RobloxApi::Catalog,
			format!("/v1/favorites/users/{user_id}/bundles/{bundle_id}/favorite"),
		)
		.method(Method::POST)
		.send::<EmptyResponse>()
		.await?;

		Ok(())
	}

	pub async fn unfavorite_bundle(&self, user_id: u64, bundle_id: u64) -> Result<(), RoboltError> {
		self.request(
			RobloxApi::Catalog,
			format!("/v1/favorites/users/{user_id}/bundles/{bundle_id}/favorite"),
		)
		.method(Method::DELETE)
		.send::<EmptyResponse>()
		.await?;

		Ok(())
	}

	pub async fn asset_favorite_model(
		&self,
		user_id: u64,
		asset_id: u64,
	) -> Result<Option<AssetFavorite>, RoboltError> {
		self.request(
			RobloxApi::Catalog,
			format!("/v1/favorites/users/{user_id}/assets/{asset_id}/favorite"),
		)
		.send()
		.await
	}

	pub async fn bundle_favorite_model(
		&self,
		user_id: u64,
		bundle_id: u64,
	) -> Result<Option<BundleFavorite>, RoboltError> {
		self.request(
			RobloxApi::Catalog,
			format!("/v1/favorites/users/{user_id}/bundles/{bundle_id}/favorite"),
		)
		.send()
		.await
	}

	pub async fn favorite_bundles(
		&self,
		user_id: u64,
		subcategory: CatalogSubcategory,
		limit: Limit,
		page: u32,
	) -> Result<FavoriteBundles, RoboltError> {
		self.request(
			RobloxApi::Catalog,
			format!(
				"/v1/favorites/users/{user_id}/favorites/{}/bundles?pageNumber={page}&itemsPerPage={}",
				subcategory as u8, limit as u8
			),
		)
		.send()
		.await
	}

	pub async fn item(&self, items: Vec<CatalogSearchItem>) -> Result<Vec<CatalogSearchResponse>, RoboltError> {
		let body = json!({ "items": items });

		self.request(RobloxApi::Catalog, "/v1/catalog/items/details")
			.method(Method::POST)
			.send_body::<_, DataResponse<CatalogSearchResponse>>(body)
			.await
			.map(|res| res.data)
	}
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogSearchItem {
	pub id: u64,
	pub item_type: ItemType,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogSearchResponse {
	pub id: u64,
	pub item_type: ItemType,
	pub asset_type: Option<AssetType>,
	pub bundle_type: Option<BundleType>,
	pub name: String,
	pub description: String,
	pub product_id: u64,
	pub genres: Option<Vec<CatalogItemGenre>>,
	pub bundled_items: Option<Vec<BundleItem>>,
	pub item_status: Vec<CatalogItemStatus>,
	pub item_restrictions: Vec<CatalogItemRestriction>,
	pub creator_has_verified_badge: bool,
	pub creator_type: CreatorType,
	pub creator_target_id: u64,
	pub creator_name: String,
	pub price: Option<u64>,
	pub premium_pricing: Option<PremiumPricing>,
	pub lowest_price: Option<u64>,
	pub price_status: Option<String>,
	pub units_available_for_consumption: Option<u64>,
	pub purchase_count: Option<u64>,
	pub favorite_count: u64,
	pub off_sale_deadline: Option<String>,
	pub collectible_item_id: Option<String>,
	pub total_quantity: Option<u64>,
	pub sale_location_type: SaleLocationType,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum CreatorType {
	Group,
	User,
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum AssetType {
	Image = 1,
	TShirt = 2,
	Audio = 3,
	Mesh = 4,
	Lua = 5,
	HTML = 6,
	Text = 7,
	Hat = 8,
	Place = 9,
	Model = 10,
	Shirt = 11,
	Pants = 12,
	Decal = 13,
	Avatar = 16,
	Head = 17,
	Face = 18,
	Gear = 19,
	Badge = 21,
	GroupEmblem = 22,
	Animation = 24,
	Arms = 25,
	Legs = 26,
	Torso = 27,
	RightArm = 28,
	LeftArm = 29,
	LeftLeg = 30,
	RightLeg = 31,
	Package = 32,
	YouTubeVideo = 33,
	GamePass = 34,
	App = 35,
	Code = 37,
	Plugin = 38,
	SolidModel = 39,
	MeshPart = 40,
	HairAccessory = 41,
	FaceAccessory = 42,
	NeckAccessory = 43,
	ShoulderAccessory = 44,
	FrontAccessory = 45,
	BackAccessory = 46,
	WaistAccessory = 47,
	ClimbAnimation = 48,
	DeathAnimation = 49,
	FallAnimation = 50,
	IdleAnimation = 51,
	JumpAnimation = 52,
	RunAnimation = 53,
	SwimAnimation = 54,
	WalkAnimation = 55,
	PoseAnimation = 56,
	LocalizationTableManifest = 59,
	LocalizationTableTranslation = 60,
	EmoteAnimation = 61,
	Video = 62,
	TexturePack = 63,
	TShirtAccessory = 64,
	ShirtAccessory = 65,
	PantsAccessory = 66,
	JacketAccessory = 67,
	SweaterAccessory = 68,
	ShortsAccessory = 69,
	LeftShoeAccessory = 70,
	RightShoeAccessory = 71,
	DressSkirtAccessory = 72,
	FontFamily = 73,
	FontFace = 74,
	MeshHiddenSurfaceRemoval = 75,
	EyebrowAccessory = 76,
	EyelashAccessory = 77,
	MoodAnimation = 78,
	DynamicHead = 79,
	CodeSnippet = 80,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[repr(u8)]
pub enum CatalogItemGenre {
	#[default]
	All,
	Tutorial,
	Scary,
	TownAndCity,
	War,
	Funny,
	Fantasy,
	Adventure,
	SciFi,
	Pirate,
	FPS,
	RPG,
	Sports,
	Ninja,
	WildWest,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[repr(u8)]
pub enum SaleLocationType {
	NotApplicable,
	ShopOnly,
	MyExperiencesOnly,
	ShopAndMyExperiences,
	ExperiencesById,
	ShopAndAllExperiences,
	ExperiencesDevApiOnly,
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum CatalogItemStatus {
	New = 1,
	Sale = 2,
	SaleTimer = 7,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetFavorite {
	user_id: u64,
	asset_id: u64,
	created: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteBundles {
	favorites: Vec<Bundle>,
	more_favorites: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleFavorite {
	user_id: u64,
	bundle_id: u64,
	created: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
	pub id: u64,
	pub name: String,
	pub description: String,
	pub bundle_type: String,
	pub items: Vec<BundleItem>,
	pub creator: BundleCreator,
	pub product: BundleProduct,
	pub item_restrictions: Vec<CatalogItemRestriction>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedBundle {
	pub id: u64,
	pub name: String,
	pub bundle_type: String,
	pub creator: BundleCreator,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleItem {
	pub owned: Option<bool>,
	pub id: u64,
	pub name: String,
	#[serde(rename = "type")]
	pub item_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleCreator {
	pub id: u64,
	pub name: String,
	#[serde(rename = "type")]
	pub creator_type: String,
	pub has_verified_badge: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleProduct {
	pub id: u64,
	#[serde(rename = "type")]
	pub product_type: String,
	pub is_public_domain: bool,
	pub is_for_sale: bool,
	pub price_in_robux: Option<u64>,
	pub is_free: bool,
	pub no_price_text: Option<String>,
	pub premium_pricing: Option<PremiumPricing>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumPricing {
	pub premium_discount_percentage: u64,
	pub premium_price_in_robux: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum BundleType {
	#[default]
	BodyParts = 1,
	AvatarAnimations = 2,
	Shoes = 3,
	DynamicHead = 4,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ItemType {
	#[default]
	Asset = 1,
	Bundle = 2,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum CatalogItemRestriction {
	ThirteenPlus,
	LimitedUnique,
	Limited,
	BuildersClub,
	TurboBuildersClub,
	OutrageousBuildersClub,
	Rthro,
	Live,
	Collectible,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum CatalogSubcategory {
	#[default]
	Featured = 0,
	All = 1,
	Collectibles = 2,
	Clothing = 3,
	BodyParts = 4,
	Gear = 5,
	Models = 6,
	Plugins = 7,
	Decals = 8,
	Hats = 9,
	Faces = 10,
	Packages = 11,
	Shirts = 12,
	TShirts = 13,
	Pants = 14,
	Heads = 15,
	Audio = 16,
	RobloxCreated = 17,
	Meshes = 18,
	Accessories = 19,
	HairAccessories = 20,
	FaceAccessories = 21,
	NeckAccessories = 22,
	ShoulderAccessories = 23,
	FrontAccessories = 24,
	BackAccessories = 25,
	WaistAccessories = 26,
	AvatarAnimations = 27,
	ClimbAnimations = 28,
	FallAnimations = 30,
	IdleAnimations = 31,
	JumpAnimations = 32,
	RunAnimations = 33,
	SwimAnimations = 34,
	WalkAnimations = 35,
	AnimationPackage = 36,
	BodyPartsBundles = 37,
	AnimationBundles = 38,
	EmoteAnimations = 39,
	CommunityCreations = 40,
	Video = 41,
	Recommended = 51,
	LayeredClothing = 52,
	AllBundles = 53,
	HeadAccessories = 54,
	ClassicTShirts = 55,
	ClassicShirts = 56,
	ClassicPants = 57,
	TShirtAccessories = 58,
	ShirtAccessories = 59,
	PantsAccessories = 60,
	JacketAccessories = 61,
	SweaterAccessories = 62,
	ShortsAccessories = 63,
	ShoesBundles = 64,
	DressSkirtAccessories = 65,
	DynamicHeads = 66,
}