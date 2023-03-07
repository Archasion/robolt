use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_avatar() {
	let client = Robolt::new();
	assert_ok!(client.fetch_avatar(1));
}

#[test]
fn fetch_wearing_assets() {
	let client = Robolt::new();
	assert_ok!(client.fetch_currently_wearing(1));
}