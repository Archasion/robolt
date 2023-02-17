use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_badge_by_id() {
    let client = Robolt::new();
    assert_ok!(client.fetch_badge(2124789031));
}

#[test]
fn fetch_game_badges() {
    let client = Robolt::new();
    assert_ok!(client.fetch_game_badges(2680623874));
}

#[test]
fn fetch_user_badges() {
    let client = Robolt::new();
    assert_ok!(client.fetch_user_badges(2));
}

#[test]
fn fetch_user_awarded_badge_dates() {
    let client = Robolt::new();
    assert_ok!(client.fetch_user_awarded_badge_dates(2, vec![1]));
}

#[test]
fn has_badge() {
    let client = Robolt::new();
    assert!(client.has_badge(2, 276292005).unwrap());
}

#[test]
fn has_badges() {
    let client = Robolt::new();
    let badges = vec![276292005, 275629737, 276292089];

    assert!(client.has_badges(2, badges).unwrap());
}

#[test]
fn has_badges_some() {
    let client = Robolt::new();
    let badges = vec![276292005, 275629737, 2, 1];

    assert!(client.has_badges_some(2, badges).unwrap());
}