use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_presences() {
	let client = Robolt::new();
	let user_ids = vec![1, 2, 3];
	assert_ok!(client.fetch_presences(user_ids));
}

#[test]
fn fetch_last_online() {
	let client = Robolt::new();
	let user_ids = vec![1, 2, 3];
	assert_ok!(client.fetch_last_online(user_ids));
}