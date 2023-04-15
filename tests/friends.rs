use tokio_test::assert_ok;

use robolt::api::Limit;
use robolt::Robolt;

#[tokio::test]
async fn followers() {
	let client = Robolt::new();
	assert_ok!(client.followers(1, Limit::Min).await);
}

#[tokio::test]
async fn followings() {
	let client = Robolt::new();
	assert_ok!(client.followings(1, Limit::Min).await);
}

#[tokio::test]
async fn friends() {
	let client = Robolt::new();
	assert_ok!(client.friends(1).await);
}

#[tokio::test]
async fn friend_count() {
	let client = Robolt::new();
	assert_ok!(client.friend_count(1).await);
}

#[tokio::test]
async fn follower_count() {
	let client = Robolt::new();
	assert_ok!(client.follower_count(1).await);
}

#[tokio::test]
async fn following_count() {
	let client = Robolt::new();
	assert_ok!(client.following_count(1).await);
}
