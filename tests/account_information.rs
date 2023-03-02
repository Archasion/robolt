use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_roblox_badges() {
    let client = Robolt::new();
    assert_ok!(client.fetch_roblox_badges(1));
}

#[test]
fn fetch_user_socials() {
    let client = Robolt::new();
    assert_ok!(client.fetch_user_socials(1));
}
