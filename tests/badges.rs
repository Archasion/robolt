use tokio_test::assert_ok;

use robolt::api::Limit;
use robolt::Robolt;

#[tokio::test]
async fn badge_by_id() {
	let client = Robolt::new();
	assert_ok!(client.badge(2124789031).await);
}

#[tokio::test]
async fn universe_badges() {
	let client = Robolt::new();
	assert_ok!(client.universe_badges(2680623874, Limit::Min).await);
}

#[tokio::test]
async fn user_badges() {
	let client = Robolt::new();
	assert_ok!(client.user_badges(1, Limit::Min).await);
}

#[tokio::test]
async fn awarded_badge_timestamps() {
	let client = Robolt::new();
	let badges = vec![276292005, 275629737, 276292089];

	assert_ok!(client.awarded_badge_timestamps(2, badges).await);
}
