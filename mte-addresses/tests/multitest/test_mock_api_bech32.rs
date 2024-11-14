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
    use cw_multi_test::MockApiBech32;

    let addr = MockApiBech32::new(CW_PREFIX).addr_make("owner");

    assert_eq!(CW_BECH32, addr.as_str());
}

#[test]
fn rule_4() {
    // not possible
}

#[test]
fn rule_5() {
    use cw_multi_test::MockApiBech32;

    let addr = MockApiBech32::new(NB_PREFIX).addr_make("owner");

    assert_eq!(NB_BECH32, addr.as_str());
}

#[test]
fn rule_6() {
    // not possible
}
