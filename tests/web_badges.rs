use arlox::web::badges;
use tokio_test::assert_ok;

#[test]
fn fetch_badge_info() {
    assert_ok!(badges::fetch(2124789031));
}
