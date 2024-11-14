use super::*;

#[test]
fn creating_address_with_prefix_should_work() {
    use cw_multi_test::MockApiBech32;

    let addr = MockApiBech32::new("cosmwasm").addr_make("owner");

    assert_eq!(CW_BECH32, addr.as_str());
}
