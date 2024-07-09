use cw_multi_test::MockApiBech32;

#[test]
fn app_api_with_default_prefix_should_work() {
    use cw_multi_test::App;

    let app = App::default();
    let addr = app.api().addr_make("owner");
    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
        addr.as_str()
    );
}

#[test]
fn app_api_with_custom_prefix_should_work() {
    use cw_multi_test::{no_init, AppBuilder};

    let app = AppBuilder::default()
        .with_api(MockApiBech32::new("juno"))
        .build(no_init);

    let addr = app.api().addr_make("owner");
    assert_eq!(
        "juno1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsmg2ndy",
        addr.as_str()
    );
}
