use tokio_test::assert_ok;

use robolt::api::Limit;
use robolt::Robolt;

#[tokio::test]
async fn user() {
	let client = Robolt::new();
	assert_ok!(client.user(1).await);
}

#[tokio::test]
async fn partial_user() {
	let client = Robolt::new();
	assert_ok!(client.partial_user(1).await);
}

#[tokio::test]
async fn username_history() {
	let client = Robolt::new();
	assert_ok!(client.username_history(1).await);
}

#[tokio::test]
async fn search_users() {
	let client = Robolt::new();
	assert_ok!(client.search_users("test", Limit::Min).await);
}

#[tokio::test]
async fn users_from_ids() {
	let client = Robolt::new();
	assert_ok!(client.users_from_ids(vec![1, 2, 3], false).await);
}

#[tokio::test]
async fn validate_display_name() {
	let client = Robolt::new();
	assert_ok!(client.validate_display_name("test", "01-01-1999").await);
}