use tokio_test::assert_ok;

use robolt::{RobloxBadge, Robolt};

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
    assert_ok!(client.fetch_user_id("roblox"));
}

#[test]
fn fetch_username_history() {
    let client = Robolt::new();
    assert_ok!(client.fetch_username_history(1));
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

#[test]
fn fetch_roblox_badges_by_user_id() {
    let client = Robolt::new();
    assert_ok!(client.fetch_roblox_badges(1));
    dbg!(client.fetch_roblox_badges(1).unwrap());
}

#[test]
fn check_for_roblox_badge() {
    let client = Robolt::new();
    assert_ok!(client.has_roblox_badge(1, RobloxBadge::Administrator));
}

#[test]
fn validate_display_name() {
    let client = Robolt::new();
    assert_ok!(client.validate_display_name("test", "01-01-1999"));
}
