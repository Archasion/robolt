use dotenv::dotenv;
use tokio_test::{assert_err, assert_ok};
use robolt::Robolt;

#[test]
fn fetch_user_by_id() {
    let client = Robolt::new();
    assert_ok!(client.users.fetch(1));
}

#[test]
fn authenticated_user() {
    dotenv().ok();
    let cookie = std::env::var("ROBLOX_COOKIE").unwrap();
    let mut client = Robolt::new();

    assert_ok!(client.login(cookie));
    assert_ok!(client.users.me());

    client.logout();

    assert_err!(client.users.me());
}