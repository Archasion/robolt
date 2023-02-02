use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_user_by_id() {
    let client = Robolt::new();
    assert_ok!(client.users.fetch(1));
}

#[test]
fn fetch_partial_user_by_id() {
    let client = Robolt::new();
    assert_ok!(client.users.partial(1));
}

#[test]
fn find_user_by_username() {
    let client = Robolt::new();
    assert_ok!(client.users.find("roblox"));
}

#[test]
fn check_username_history() {
    let client = Robolt::new();
    assert_ok!(client.users.username_history(1));
}

#[test]
fn search_users_by_keyword() {
    let client = Robolt::new();
    assert_ok!(client.users.search("test", 10));
}