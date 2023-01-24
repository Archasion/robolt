use tokio_test::{assert_err};
use arlox::web::auth;

#[test]
fn authenticate_user_with_invalid_cookie() {
    assert_err!(auth::login(""));
}