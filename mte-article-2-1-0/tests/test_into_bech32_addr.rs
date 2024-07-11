#[test]
fn into_bech32_address_should_work() {
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32();
    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
        addr.as_str()
    );
}

#[test]
fn into_bech32_with_custom_prefix_should_work() {
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32_with_prefix("juno");
    assert_eq!(
        "juno1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsmg2ndy",
        addr.as_str()
    );
}
