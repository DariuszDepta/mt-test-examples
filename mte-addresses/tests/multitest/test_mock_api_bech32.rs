#[test]
fn rule_1() {
    // not possible
}

#[test]
fn rule_2() {
    // not possible
}

#[test]
fn rule_3() {
    use cw_multi_test::MockApiBech32;

    let addr = MockApiBech32::new("cosmwasm").addr_make("owner");

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
    use cw_multi_test::MockApiBech32;

    let addr = MockApiBech32::new("nebula").addr_make("owner");

    assert_eq!(
        "nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsvsqrvp",
        addr.as_str()
    );
}

#[test]
fn rule_6() {
    // not possible
}
