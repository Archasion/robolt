use tokio_test::assert_ok;

use robolt::Robolt;

#[tokio::test]
async fn fetch_points() {
	let client = Robolt::new();
	assert_ok!(client.fetch_points(1, 2680623874).await);
}