use super::*;

#[test]
fn rule_1() {
    // not possible
}

#[test]
fn rule_2() {
    // not possible
}

#[test]
fn rule_3() {
    // not possible
}

#[test]
fn rule_4() {
    use cw_multi_test::MockApiBech32m;

    let addr = MockApiBech32m::new(CW_PREFIX).addr_make("owner");

    assert_eq!(CW_BECH32M, addr.as_str());
}

#[test]
fn rule_5() {
    // not possible
}

#[test]
fn rule_6() {
    use cw_multi_test::MockApiBech32m;

    let addr = MockApiBech32m::new(NB_PREFIX).addr_make("owner");

    assert_eq!(NB_BECH32M, addr.as_str());
}
