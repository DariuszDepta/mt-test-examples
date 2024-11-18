#[test]
fn rule_1() {
    use cosmwasm_std::testing::MockApi;

    let addr = MockApi::default().addr_make("owner");

    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
        addr.as_str()
    );
}

#[test]
fn rule_2() {
    // not possible
}

#[test]
fn rule_3() {
    use cosmwasm_std::testing::MockApi;

    let addr = MockApi::default()
        .with_prefix("cosmwasm")
        .addr_make("owner");

    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
        addr.as_str()
    );
}

#[test]
fn rule_4() {
    // not possible
}

#[test]
fn rule_5() {
    use cosmwasm_std::testing::MockApi;

    let addr = MockApi::default().with_prefix("nebula").addr_make("owner");

    assert_eq!(
        "nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsvsqrvp",
        addr.as_str()
    );
}

#[test]
fn rule_6() {
    // not possible
}
