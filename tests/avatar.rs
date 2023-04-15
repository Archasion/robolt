use tokio_test::assert_ok;

use robolt::{Limit, Robolt};

#[tokio::test]
async fn fetch_avatar() {
	let client = Robolt::new();
	assert_ok!(client.fetch_avatar(1).await);
}

#[tokio::test]
async fn fetch_currently_wearing() {
	let client = Robolt::new();
	assert_ok!(client.fetch_currently_wearing(1).await);
}

#[tokio::test]
async fn fetch_outfits() {
	let client = Robolt::new();
	let res = client
		.fetch_outfits(1)
		.items_per_page(Limit::default())
		.page(1)
		.editable(true)
		.send()
		.await;

	assert_ok!(res);
}

#[tokio::test]
async fn fetch_avatar_metadata() {
	let client = Robolt::new();
	assert_ok!(client.fetch_avatar_metadata().await);
}

#[tokio::test]
async fn fetch_game_start_info() {
	let client = Robolt::new();
	assert_ok!(client.fetch_game_start_info(2124789031).await);
}

#[tokio::test]
async fn fetch_outfit() {
	let client = Robolt::new();
	assert_ok!(client.fetch_outfit(11675594890).await);
}
