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
    // not possible
}

#[test]
fn rule_4() {
    use cw_multi_test::MockApiBech32m;

    let addr = MockApiBech32m::new("cosmwasm").addr_make("owner");

    assert_eq!(
        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsl5lc5x",
        addr.as_str()
    );
}

#[test]
fn rule_5() {
    // not possible
}

#[test]
fn rule_6() {
    use cw_multi_test::MockApiBech32m;

    let addr = MockApiBech32m::new("nebula").addr_make("owner");

    assert_eq!(
        "nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsevs0fr",
        addr.as_str()
    );
}
