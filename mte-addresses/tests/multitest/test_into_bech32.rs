#[test]
fn rule_1() {
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32();

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
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32_with_prefix("cosmwasm");

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
    use cw_multi_test::IntoBech32;

    let addr = "owner".into_bech32_with_prefix("nebula");

    assert_eq!(
        "nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsvsqrvp",
        addr.as_str()
    );
}

#[test]
fn rule_6() {
    // not possible
}
