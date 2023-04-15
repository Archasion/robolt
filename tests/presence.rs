use tokio_test::assert_ok;

use robolt::Robolt;

#[tokio::test]
async fn presences() {
	let client = Robolt::new();
	let user_ids = vec![1, 2, 3];
	assert_ok!(client.presences(user_ids).await);
}

#[tokio::test]
async fn partial_presences() {
	let client = Robolt::new();
	let user_ids = vec![1, 2, 3];
	assert_ok!(client.partial_presences(user_ids).await);
}
