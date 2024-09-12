#[test]
fn creating_address_with_prefix_should_work() {
    use cw_multi_test::MockApiBech32m;

    let addr = MockApiBech32m::new("cosmwasm").addr_make("owner");
    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsl5lc5x",
        addr.as_str()
    );
}
