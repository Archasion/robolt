use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_badge_by_id() {
    let roblox = Robolt::new();
    assert_ok!(roblox.fetch_badge(2124789031));
}