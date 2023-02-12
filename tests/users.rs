use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_user_by_id() {
    let client = Robolt::new();
    assert_ok!(client.fetch_user(1));
}

#[test]
fn fetch_partial_user_by_id() {
    let client = Robolt::new();
    assert_ok!(client.fetch_partial_user(1));
}

#[test]
fn find_id_by_username() {
    let client = Robolt::new();
    assert_ok!(client.user_id("roblox"));
}

#[test]
fn fetch_username_history() {
    let client = Robolt::new();
    assert_ok!(client.username_history(1));
}

#[test]
fn search_users_by_keyword() {
    let client = Robolt::new();
    assert_ok!(client.search_users("test", 10));
}

#[test]
fn search_users_by_ids() {
    let client = Robolt::new();
    assert_ok!(client.fetch_users(vec![1, 2, 3], false));
}
