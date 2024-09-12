#[test]
fn creating_address_with_prefix_should_work() {
    use cw_multi_test::MockApiBech32;

    let addr = MockApiBech32::new("cosmwasm").addr_make("owner");
    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
        addr.as_str()
    );
}
