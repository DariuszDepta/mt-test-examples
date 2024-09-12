#[test]
fn creating_address_with_default_prefix_should_work() {
    use cw_multi_test::App;

    let app = App::default();

    let addr = app.api().addr_make("owner");
    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
        addr.as_str()
    );
}

#[test]
fn creating_bech32_address_with_custom_prefix_should_work() {
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32};

    let app = AppBuilder::default()
        .with_api(MockApiBech32::new("juno"))
        .build(no_init);

    let addr = app.api().addr_make("owner");
    assert_eq!(
        "juno1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsmg2ndy",
        addr.as_str()
    );
}

#[test]
fn creating_bech32m_address_with_custom_prefix_should_work() {
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32m};

    let app = AppBuilder::default()
        .with_api(MockApiBech32m::new("juno"))
        .build(no_init);

    let addr = app.api().addr_make("owner");
    assert_eq!(
        "juno1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsw56lgx",
        addr.as_str()
    );
}
