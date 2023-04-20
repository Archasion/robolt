use tokio_test::assert_ok;

use robolt::api::catalog::BundleType;
use robolt::api::Limit;
use robolt::Robolt;

#[tokio::test]
async fn asset_parent_bundles() {
	let client = Robolt::new();
	assert_ok!(client.asset_parent_bundles(3333105280, Limit::Min).await);
}

#[tokio::test]
async fn bundle() {
	let client = Robolt::new();
	assert_ok!(client.bundle(495).await);
}

#[tokio::test]
async fn bundle_recommendations() {
	let client = Robolt::new();
	assert_ok!(client.bundle_recommendations(495, 5).await);
}

#[tokio::test]
async fn bundles() {
	let client = Robolt::new();
	let bundle_ids = vec![1, 2, 3];
	assert_ok!(client.bundles(bundle_ids).await);
}

#[tokio::test]
async fn user_bundles() {
	let client = Robolt::new();
	assert_ok!(client.user_bundles(1).await);
}

#[tokio::test]
async fn user_bundles_filter() {
	let client = Robolt::new();
	assert_ok!(client.user_bundles_filter(1, BundleType::BodyParts, Limit::Min).await);
}

#[tokio::test]
async fn asset_favorite_count() {
	let client = Robolt::new();
	assert_ok!(client.asset_favorite_count(3333105280).await);
}

#[tokio::test]
async fn bundle_favorite_count() {
	let client = Robolt::new();
	assert_ok!(client.bundle_favorite_count(495).await);
}