use tokio_test::assert_ok;

use robolt::{RobloxBadge, Robolt};

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
    assert_ok!(client.fetch_user_badges(1));
}

#[test]
fn fetch_user_awarded_badge_dates() {
    let client = Robolt::new();
    let badges = vec![276292005, 275629737, 276292089];

    assert_ok!(client.fetch_awarded_timestamps(2, badges));
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
fn has_badges_any() {
    let client = Robolt::new();
    let badges = vec![276292005, 275629737, 2, 1];

    assert!(client.has_badges_any(2, badges).unwrap());
}

#[test]
fn has_roblox_badge() {
    let client = Robolt::new();
    assert!(client
        .has_roblox_badge(1, RobloxBadge::Administrator)
        .unwrap());
}

#[test]
fn has_roblox_badges() {
    let client = Robolt::new();
    let badges = vec![
        RobloxBadge::Administrator,
        RobloxBadge::Bloxxer,
        RobloxBadge::Warrior,
        RobloxBadge::Veteran,
    ];

    assert!(client.has_roblox_badges(1, badges).unwrap());
}

#[test]
fn has_roblox_badges_any() {
    let client = Robolt::new();
    let badges = vec![RobloxBadge::Administrator, RobloxBadge::Ambassador];

    assert!(client.has_roblox_badges_any(1, badges).unwrap());
}
