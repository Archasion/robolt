use dotenv::dotenv;
use robolt::web::badges;
use robolt::web::badges::BadgeConfig;
use tokio_test::{assert_err, assert_ok};
use robolt::web::auth::login;
use std::env;

#[test]
fn fetch_badge_info() {
    assert_ok!(badges::fetch(2124789031));
}

#[test]
fn fetch_badge_info_invalid_id() {
    assert_err!(badges::fetch(0));
}

#[test]
fn err_update_badge() {
    let config = BadgeConfig {
        name: None,
        description: None,
        enabled: true,
        return_updated_badge: false,
    };

    assert_err!(badges::update(2124789031, config));
}

#[test]
fn err_remove_badge() {
    assert_err!(badges::remove(2124789031));
}