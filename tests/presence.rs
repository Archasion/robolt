use tokio_test::assert_ok;

use robolt::Robolt;

#[tokio::test]
async fn fetch_presences() {
	let client = Robolt::new();
	let user_ids = vec![1, 2, 3];
	assert_ok!(client.fetch_presences(user_ids).await);
}

#[tokio::test]
async fn fetch_partial_presences() {
	let client = Robolt::new();
	let user_ids = vec![1, 2, 3];
	assert_ok!(client.fetch_partial_presences(user_ids).await);
}