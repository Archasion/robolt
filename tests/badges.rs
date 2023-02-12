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