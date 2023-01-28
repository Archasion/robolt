use dotenv::dotenv;
use tokio_test::{assert_err, assert_ok};

use robolt::Robolt;

#[test]
fn fetch_user_by_id() {
    let client = Robolt::new();
    assert_ok!(client.users.fetch(1));
}

#[test]
fn find_user_by_username() {
    let client = Robolt::new();
    assert_ok!(client.users.find("roblox"));
}