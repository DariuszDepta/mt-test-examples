use super::*;

#[test]
fn creating_address_should_work() {
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32();

    assert_eq!(CW_BECH32, addr.as_str());
}

#[test]
fn creating_address_with_default_prefix_should_work() {
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32_with_prefix(CW_PREFIX);

    assert_eq!(CW_BECH32, addr.as_str());
}

#[test]
fn creating_address_with_custom_prefix_should_work() {
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32_with_prefix(NB_PREFIX);

    assert_eq!(NB_BECH32, addr.as_str());
}
