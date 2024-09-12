#[test]
fn creating_address_with_default_prefix_should_work() {
    use cw_multi_test::IntoBech32m;

    let addr = "owner".into_bech32m();
    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsl5lc5x",
        addr.as_str()
    );
}

#[test]
fn creating_address_with_custom_prefix_should_work() {
    use cw_multi_test::IntoBech32m;

    let addr = "owner".into_bech32m_with_prefix("juno");
    assert_eq!(
        "juno1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsw56lgx",
        addr.as_str()
    );
}
