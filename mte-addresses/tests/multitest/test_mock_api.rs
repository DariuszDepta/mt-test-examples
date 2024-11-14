use super::*;

#[test]
fn rule_1() {
    use cosmwasm_std::testing::MockApi;

    let addr = MockApi::default().addr_make("owner");

    assert_eq!(CW_BECH32, addr.as_str());
}

#[test]
fn rule_2() {
    // not possible
}

#[test]
fn rule_3() {
    use cosmwasm_std::testing::MockApi;

    let addr = MockApi::default().with_prefix(CW_PREFIX).addr_make("owner");

    assert_eq!(CW_BECH32, addr.as_str());
}

#[test]
fn rule_4() {
    // not possible
}

#[test]
fn rule_5() {
    use cosmwasm_std::testing::MockApi;

    let addr = MockApi::default().with_prefix(NB_PREFIX).addr_make("owner");

    assert_eq!(NB_BECH32, addr.as_str());
}

#[test]
fn rule_6() {
    // not possible
}
