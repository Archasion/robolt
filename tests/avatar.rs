use tokio_test::assert_ok;

use robolt::api::Limit;
use robolt::Robolt;

#[tokio::test]
async fn avatar() {
	let client = Robolt::new();
	assert_ok!(client.avatar(1).await);
}

#[tokio::test]
async fn currently_wearing() {
	let client = Robolt::new();
	assert_ok!(client.currently_wearing(1).await);
}

#[tokio::test]
async fn outfits() {
	let client = Robolt::new();
	let res = client
		.outfits(1)
		.items_per_page(Limit::default())
		.page(1)
		.editable(true)
		.send()
		.await;

	assert_ok!(res);
}

#[tokio::test]
async fn avatar_metadata() {
	let client = Robolt::new();
	assert_ok!(client.avatar_metadata().await);
}

#[tokio::test]
async fn game_start_info() {
	let client = Robolt::new();
	assert_ok!(client.game_start_info(2124789031).await);
}

#[tokio::test]
async fn outfit() {
	let client = Robolt::new();
	assert_ok!(client.outfit(11675594890).await);
}
