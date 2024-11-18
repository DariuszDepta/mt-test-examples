#[test]
fn rule_1() {
    use cw_multi_test::App;

    let app = App::default();

    let addr = app.api().addr_make("owner");

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
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32};

    let app = AppBuilder::default()
        .with_api(MockApiBech32::new("cosmwasm"))
        .build(no_init);

    let addr = app.api().addr_make("owner");

    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
        addr.as_str()
    );
}

#[test]
fn rule_4() {
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32m};

    let app = AppBuilder::default()
        .with_api(MockApiBech32m::new("cosmwasm"))
        .build(no_init);

    let addr = app.api().addr_make("owner");

    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsl5lc5x",
        addr.as_str()
    );
}

#[test]
fn rule_5() {
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32};

    let app = AppBuilder::default()
        .with_api(MockApiBech32::new("nebula"))
        .build(no_init);

    let addr = app.api().addr_make("owner");

    assert_eq!(
        "nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsvsqrvp",
        addr.as_str()
    );
}

#[test]
fn rule_6() {
    use cw_multi_test::{no_init, AppBuilder, MockApiBech32m};

    let app = AppBuilder::default()
        .with_api(MockApiBech32m::new("nebula"))
        .build(no_init);

    let addr = app.api().addr_make("owner");

    assert_eq!(
        "nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsevs0fr",
        addr.as_str()
    );
}

#[test]
#[allow(unused_variables)]
fn init_1() {
    use cw_multi_test::App;

    let app = App::new(|router, api, storage| {
        let addr = api.addr_make("owner");
        assert_eq!(
            "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
            addr.as_str()
        );
    });
}

#[test]
#[allow(unused_variables)]
fn init_2() {
    use cw_multi_test::{AppBuilder, MockApiBech32};

    let app = AppBuilder::default()
        .with_api(MockApiBech32::new("nebula"))
        .build(|router, api, storage| {
            let addr = api.addr_make("owner");
            assert_eq!(
                "nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsvsqrvp",
                addr.as_str()
            );
        });
}

#[test]
#[allow(unused_variables)]
fn init_3() {
    use cw_multi_test::{AppBuilder, MockApiBech32m};

    let app = AppBuilder::default()
        .with_api(MockApiBech32m::new("nebula"))
        .build(|router, api, storage| {
            let addr = api.addr_make("owner");
            assert_eq!(
                "nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsevs0fr",
                addr.as_str()
            );
        });
}

#[test]
#[allow(unused_variables)]
fn init_4() {
    use cw_multi_test::{AppBuilder, MockApiBech32m};

    let app = AppBuilder::default()
        .with_api(MockApiBech32m::new("cosmwasm"))
        .build(|router, api, storage| {
            let addr = api.addr_make("owner");
            assert_eq!(
                "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsl5lc5x",
                addr.as_str()
            );
        });
}
