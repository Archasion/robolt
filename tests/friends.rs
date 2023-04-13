use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_followers() {
	let client = Robolt::new();
	assert_ok!(client.fetch_followers(1, 10));
}

#[test]
fn fetch_followings() {
	let client = Robolt::new();
	assert_ok!(client.fetch_followings(1, 10));
}

#[test]
fn fetch_friends() {
	let client = Robolt::new();
	assert_ok!(client.fetch_friends(1));
}

#[test]
fn fetch_friend_count() {
	let client = Robolt::new();
	assert_ok!(client.fetch_friend_count(1));
}

#[test]
fn fetch_follower_count() {
	let client = Robolt::new();
	assert_ok!(client.fetch_follower_count(1));
}

#[test]
fn fetch_following_count() {
	let client = Robolt::new();
	assert_ok!(client.fetch_following_count(1));
}