use super::*;

#[test]
fn rule_1() {
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32();

    assert_eq!(CW_BECH32, addr.as_str());
}

#[test]
fn rule_2() {
    // not possible
}

#[test]
fn rule_3() {
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32_with_prefix(CW_PREFIX);

    assert_eq!(CW_BECH32, addr.as_str());
}

#[test]
fn rule_4() {
    // not possible
}

#[test]
fn rule_5() {
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32_with_prefix(NB_PREFIX);

    assert_eq!(NB_BECH32, addr.as_str());
}

#[test]
fn rule_6() {
    // not possible
}
