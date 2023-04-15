use tokio_test::assert_ok;

use robolt::Robolt;

#[tokio::test]
async fn roblox_badges() {
	let client = Robolt::new();
	assert_ok!(client.roblox_badges(1).await);
}

#[tokio::test]
async fn user_socials() {
	let client = Robolt::new();
	assert_ok!(client.user_socials(1).await);
}
