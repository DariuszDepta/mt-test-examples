use super::*;

#[test]
fn creating_address_with_prefix_should_work() {
    use cw_multi_test::MockApiBech32m;

    let addr = MockApiBech32m::new("cosmwasm").addr_make("owner");

    assert_eq!(CW_BECH32M, addr.as_str());
}
