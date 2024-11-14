use super::*;

#[test]
fn creating_address_should_work() {
    use cw_multi_test::IntoBech32m;

    let addr = "owner".into_bech32m();

    assert_eq!(CW_BECH32M, addr.as_str());
}

#[test]
fn creating_address_with_default_prefix_should_work() {
    use cw_multi_test::IntoBech32m;

    let addr = "owner".into_bech32m_with_prefix(CW_PREFIX);

    assert_eq!(CW_BECH32M, addr.as_str());
}

#[test]
fn creating_address_with_custom_prefix_should_work() {
    use cw_multi_test::IntoBech32m;

    let addr = "owner".into_bech32m_with_prefix(NB_PREFIX);

    assert_eq!(NB_BECH32M, addr.as_str());
}
