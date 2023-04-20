use serde::Deserialize;

use crate::api::routes::RobloxApi;
use crate::api::Limit;
use crate::errors::RoboltError;
use crate::utils::response::DataResponse;
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
	pub asset_type: String,
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

#[derive(Default, Debug, Clone, PartialEq)]
pub enum BundleType {
	#[default]
	BodyParts = 1,
	AvatarAnimations = 2,
	Shoes = 3,
	DynamicHead = 4,
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