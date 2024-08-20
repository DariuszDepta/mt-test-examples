use cw_multi_test::{no_init, App};

#[test]
fn creating_default_app_should_work() {
    let app = App::default();
    assert_eq!(12345, app.block_info().height);
}

#[test]
fn creating_new_app_should_work() {
    let app = App::new(no_init);
    assert_eq!(12345, app.block_info().height);
}
