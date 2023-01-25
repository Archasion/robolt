use arlox::web::users::{self, UserMethods};
use tokio_test::{assert_err, assert_ok};

#[test]
fn fetch_user_by_id() {
    assert_ok!(users::fetch(1));
}

#[test]
fn fetch_user_by_invalid_id() {
    assert_err!(users::fetch(0));
}

#[test]
fn fetch_partial_user_by_id() {
    assert_ok!(users::partial(1));
}

#[test]
fn fetch_partial_user_by_invalid_id() {
    assert_err!(users::partial(0));
}

#[test]
fn find_user_by_username() {
    assert_ok!(users::find("roblox"));
}

#[test]
fn find_user_by_invalid_username() {
    assert_err!(users::find(""));
}

#[test]
fn check_username_history() {
    assert_ok!(users::username_history(1));
}

#[test]
fn check_username_history_of_user() {
    let user = users::partial(1).unwrap();
    assert_eq!(user.username_history().len(), 0);
}

#[test]
fn search_users() {
    assert_ok!(users::search("roblox"));
}
