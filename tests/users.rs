use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_user() {
	let client = Robolt::new();
	assert_ok!(client.fetch_user(1));
}

#[test]
fn fetch_partial_user() {
	let client = Robolt::new();
	assert_ok!(client.fetch_partial_user(1));
}

#[test]
fn fetch_username_history() {
	let client = Robolt::new();
	assert_ok!(client.fetch_username_history(1));
}

#[test]
fn search_users() {
	let client = Robolt::new();
	assert_ok!(client.search_users("test", 10));
}

#[test]
fn fetch_users_by_ids() {
	let client = Robolt::new();
	assert_ok!(client.fetch_users_by_ids(vec![1, 2, 3], false));
}

#[test]
fn validate_display_name() {
	let client = Robolt::new();
	assert_ok!(client.validate_display_name("test", "01-01-1999"));
}