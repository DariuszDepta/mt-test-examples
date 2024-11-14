//! ```text
//! 1 | app.api()  | App::default()        |       -        |     -     | CW_BECH32  | T |
//! 2 | app.api()  | App::default()        |       -        |     -     | CW_BECH32M | N |
//! 3 | app.api()  | AppBuilder.with_api() | MockApiBech32  | CW_PREFIX | CW_BECH32  | T |
//! 4 | app.api()  | AppBuilder.with_api() | MockApiBech32m | CW_PREFIX | CW_BECH32M | T |
//! 5 | app.api()  | AppBuilder.with_api() | MockApiBech32  | NB_PREFIX | NB_BECH32  | T |
//! 6 | app.api()  | AppBuilder.with_api() | MockApiBech32m | NB_PREFIX | NB_BECH32M | T |
//! ```

use super::*;

#[test]
fn rule_1() {
    use cw_multi_test::App;

    let app = App::default();

    let addr = app.api().addr_make("owner");

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
