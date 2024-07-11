#[test]
fn address_with_cosmwasm_prefix_should_work() {
    use cw_multi_test::MockApiBech32;

    let addr = MockApiBech32::new("cosmwasm").addr_make("owner");
    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
        addr.as_str()
    );
}

#[test]
fn address_with_juno_prefix_should_work() {
    use cw_multi_test::MockApiBech32;

    let addr = MockApiBech32::new("juno").addr_make("owner");
    assert_eq!(
        "juno1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsmg2ndy",
        addr.as_str()
    );
}
