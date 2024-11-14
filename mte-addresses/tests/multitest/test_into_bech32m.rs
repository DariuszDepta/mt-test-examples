use super::*;

#[test]
fn rule_1() {
    // not possible
}

#[test]
fn rule_2() {
    use cw_multi_test::IntoBech32m;

    let addr = "owner".into_bech32m();

    assert_eq!(CW_BECH32M, addr.as_str());
}

#[test]
fn rule_3() {
    // not possible
}

#[test]
fn rule_4() {
    use cw_multi_test::IntoBech32m;

    let addr = "owner".into_bech32m_with_prefix(CW_PREFIX);

    assert_eq!(CW_BECH32M, addr.as_str());
}

#[test]
fn rule_5() {
    // not possible
}

#[test]
fn rule_6() {
    use cw_multi_test::IntoBech32m;

    let addr = "owner".into_bech32m_with_prefix(NB_PREFIX);

    assert_eq!(NB_BECH32M, addr.as_str());
}
