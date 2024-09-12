#[test]
fn creating_address_using_mock_api_should_work() {
    use cosmwasm_std::testing::MockApi;

    let addr = MockApi::default().addr_make("owner");
    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
        addr.as_str()
    );
}

#[test]
fn creating_address_with_prefix_using_mock_api_should_work() {
    use cosmwasm_std::testing::MockApi;

    let addr = MockApi::default().with_prefix("juno").addr_make("owner");
    assert_eq!(
        "juno1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsmg2ndy",
        addr.as_str()
    );
}
