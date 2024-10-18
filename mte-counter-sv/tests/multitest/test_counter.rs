use counter::contract::sv::mt::{CodeId, CounterContractProxy};
use counter::msg::CounterInitMsg;
use sylvia::cw_multi_test::IntoAddr;
use sylvia::multitest::App;

#[test]
fn instantiating_with_zero_should_work() {
    let app = App::default();

    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();

    let contract = code_id
        .instantiate(CounterInitMsg::Zero)
        .call(&owner)
        .unwrap();

    assert_eq!(0, contract.value().unwrap().value);
}

#[test]
fn instantiating_with_value_should_work() {
    let app = App::default();

    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();

    let contract = code_id
        .instantiate(CounterInitMsg::Set(12))
        .call(&owner)
        .unwrap();

    assert_eq!(12, contract.value().unwrap().value);
}

#[test]
fn incrementing_should_work() {
    let app = App::default();

    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();

    let contract = code_id
        .instantiate(CounterInitMsg::Zero)
        .call(&owner)
        .unwrap();

    contract.inc().call(&owner).unwrap();

    assert_eq!(1, contract.value().unwrap().value);
}

#[test]
fn incrementing_should_stop_at_maximum() {
    let app = App::default();

    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();

    let contract = code_id
        .instantiate(CounterInitMsg::Set(250))
        .call(&owner)
        .unwrap();

    for _ in 1..=10 {
        contract.inc().call(&owner).unwrap();
    }

    assert_eq!(255, contract.value().unwrap().value);
}

#[test]
fn decrementing_should_work() {
    let app = App::default();

    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();

    let contract = code_id
        .instantiate(CounterInitMsg::Set(126))
        .call(&owner)
        .unwrap();

    contract.dec().call(&owner).unwrap();

    assert_eq!(125, contract.value().unwrap().value);
}

#[test]
fn decrementing_should_stop_at_minimum() {
    let app = App::default();

    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();

    let contract = code_id
        .instantiate(CounterInitMsg::Set(5))
        .call(&owner)
        .unwrap();

    for _ in 1..=10 {
        contract.dec().call(&owner).unwrap();
    }

    assert_eq!(0, contract.value().unwrap().value);
}

#[test]
fn setting_value_should_work() {
    let app = App::default();

    let code_id = CodeId::store_code(&app);

    let owner = "owner".into_addr();

    let contract = code_id
        .instantiate(CounterInitMsg::Set(5))
        .call(&owner)
        .unwrap();

    contract.set(125).call(&owner).unwrap();

    assert_eq!(125, contract.value().unwrap().value);
}
