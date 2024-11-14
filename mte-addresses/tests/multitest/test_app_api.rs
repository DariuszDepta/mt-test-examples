use super::*;

#[test]
fn rule_1() {
    use cw_multi_test::App;

    let app = App::default();

    let addr = app.api().addr_make("owner");

    assert_eq!(CW_BECH32, addr.as_str());
}

#[test]
fn rule_1a() {
    use cw_multi_test::App;

    let addr = App::default().api().addr_make("owner");

    assert_eq!(CW_BECH32, addr.as_str());
}

#[test]
fn rule_2() {
    // not possible
}

#[test]
fn rule_3() {
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32};

    let app = AppBuilder::default()
        .with_api(MockApiBech32::new(CW_PREFIX))
        .build(no_init);

    let addr = app.api().addr_make("owner");

    assert_eq!(CW_BECH32, addr.as_str());
}

#[test]
fn rule_4() {
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32m};

    let app = AppBuilder::default()
        .with_api(MockApiBech32m::new(CW_PREFIX))
        .build(no_init);

    let addr = app.api().addr_make("owner");

    assert_eq!(CW_BECH32M, addr.as_str());
}

#[test]
fn rule_5() {
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32};

    let app = AppBuilder::default()
        .with_api(MockApiBech32::new(NB_PREFIX))
        .build(no_init);

    let addr = app.api().addr_make("owner");

    assert_eq!(NB_BECH32, addr.as_str());
}

#[test]
fn rule_6() {
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32m};

    let app = AppBuilder::default()
        .with_api(MockApiBech32m::new(NB_PREFIX))
        .build(no_init);

    let addr = app.api().addr_make("owner");

    assert_eq!(NB_BECH32M, addr.as_str());
}

#[test]
fn rule_6a() {
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32m};

    let addr = AppBuilder::default()
        .with_api(MockApiBech32m::new(NB_PREFIX))
        .build(no_init)
        .api()
        .addr_make("owner");

    assert_eq!(NB_BECH32M, addr.as_str());
}
