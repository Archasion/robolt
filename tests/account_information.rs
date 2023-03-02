use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_roblox_badges() {
    let client = Robolt::new();
    assert_ok!(client.fetch_roblox_badges(1));
}