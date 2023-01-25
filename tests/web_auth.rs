use robolt::web::auth;
use tokio_test::assert_err;

#[test]
fn authenticate_user_with_invalid_cookie() {
    assert_err!(auth::login(""));
}