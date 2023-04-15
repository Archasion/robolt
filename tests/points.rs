use tokio_test::assert_ok;

use robolt::Robolt;

#[tokio::test]
async fn points() {
	let client = Robolt::new();
	assert_ok!(client.points(1, 2680623874).await);
}
