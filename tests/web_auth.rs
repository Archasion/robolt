use tokio_test::{assert_ok, assert_err};
use arlox::web::auth;
use dotenv::dotenv;
use std::env;

#[test]
fn authenticate_user_with_invalid_cookie() {
    assert_err!(auth::login(""));
}