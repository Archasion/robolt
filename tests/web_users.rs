use arlox::web::users;

#[test]
fn fetch_user_by_id() {
    let user = users::fetch(1);
    assert!(user.is_ok());
}

#[test]
fn fetch_user_by_invalid_id() {
    let user = users::fetch(0);
    assert!(user.is_err());
}