use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_user_by_id() {
    let users = Robolt::new().users();
    assert_ok!(users.fetch(1));
}

#[test]
fn fetch_partial_user_by_id() {
    let users = Robolt::new().users();
    assert_ok!(users.partial(1));
}

#[test]
fn find_id_by_username() {
    let users = Robolt::new().users();
    assert_ok!(users.id("roblox"));
}

#[test]
fn fetch_username_history() {
    let users = Robolt::new().users();
    assert_ok!(users.username_history(1));
}

#[test]
fn search_users_by_keyword() {
    let users = Robolt::new().users();
    assert_ok!(users.search("test", 10));
}

#[test]
fn search_users_by_ids() {
    let users = Robolt::new().users();
    assert_ok!(users.fetch_many(vec![1, 2, 3], false));
}