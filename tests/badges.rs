use tokio_test::assert_ok;

use robolt::Robolt;

#[test]
fn fetch_badge_by_id() {
    let client = Robolt::new();
    assert_ok!(client.fetch_badge(2124789031));
}

#[test]
fn fetch_universe_badges() {
    let client = Robolt::new();
    assert_ok!(client.fetch_universe_badges(2680623874));
}

#[test]
fn fetch_user_badges() {
    let client = Robolt::new();
    assert_ok!(client.fetch_user_badges(1));
}

#[test]
fn fetch_user_awarded_badge_dates() {
    let client = Robolt::new();
    let badges = vec![276292005, 275629737, 276292089];

    assert_ok!(client.fetch_awarded_timestamps(2, badges));
}
