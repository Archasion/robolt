use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_badge_by_id() {
    let client = Robolt::new();
    assert_ok!(client.fetch_badge(2124789031));
}