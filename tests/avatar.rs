use tokio_test::assert_ok;

use robolt::{Limit, Robolt};

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

#[test]
fn fetch_outfits() {
	let client = Robolt::new();
	let res = client
		.fetch_outfits(1)
		.items_per_page(Limit::default())
		.page(1)
		.editable(true)
		.send();

	assert_ok!(res);
}

#[test]
fn fetch_avatar_metadata() {
	let client = Robolt::new();
	assert_ok!(client.fetch_avatar_metadata());
}

#[test]
fn fetch_game_start_info() {
	let client = Robolt::new();
	assert_ok!(client.fetch_game_start_info(2124789031));
}
