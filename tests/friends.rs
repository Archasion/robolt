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
fn count_friends() {
	let client = Robolt::new();
	assert_ok!(client.count_friends(1));
}

#[test]
fn count_followers() {
	let client = Robolt::new();
	assert_ok!(client.count_followers(1));
}

#[test]
fn count_followings() {
	let client = Robolt::new();
	assert_ok!(client.count_followings(1));
}