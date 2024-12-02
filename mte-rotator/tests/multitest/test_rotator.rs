use cosmwasm_std::Empty;
use cw_multi_test::{App, Contract, ContractWrapper, Executor, IntoAddr};
use rotator::msg::{RotatorExecMsg, RotatorQueryMsg, RotatorResponse};

fn rotator_contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new_with_empty(
        rotator::contract::execute,
        rotator::contract::instantiate,
        rotator::contract::query,
    ))
}

#[test]
fn instantiating_with_with_empty_message_should_work() {
    let mut app = App::default();

    let code_id = app.store_code(rotator_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(code_id, owner, &Empty {}, &[], "rotator", None)
        .unwrap();

    let response: RotatorResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &RotatorQueryMsg::Value)
        .unwrap();

    assert_eq!("0,0,0,0,0,0,0,0,0,0", response.value);
}

#[test]
fn appending_values_should_work() {
    let mut app = App::default();

    let code_id = app.store_code(rotator_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(code_id, owner.clone(), &Empty {}, &[], "rotator", None)
        .unwrap();

    for i in 1..=10 {
        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &RotatorExecMsg::Value(i, 0),
            &[],
        )
        .unwrap();
    }

    let response: RotatorResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &RotatorQueryMsg::Value)
        .unwrap();

    assert_eq!("1,2,3,4,5,6,7,8,9,10", response.value);
}

#[test]
fn overflowing_should_work() {
    let mut app = App::default();

    let code_id = app.store_code(rotator_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(code_id, owner.clone(), &Empty {}, &[], "rotator", None)
        .unwrap();

    for i in 1..=78 {
        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &RotatorExecMsg::Value(i, 0),
            &[],
        )
        .unwrap();
    }

    let response: RotatorResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &RotatorQueryMsg::Value)
        .unwrap();

    assert_eq!("69,70,71,72,73,74,75,76,77,78", response.value);
}

#[test]
fn clearing_should_work() {
    let mut app = App::default();

    let code_id = app.store_code(rotator_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(code_id, owner.clone(), &Empty {}, &[], "rotator", None)
        .unwrap();

    for i in 1..=9485 {
        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &RotatorExecMsg::Value(i, 0),
            &[],
        )
        .unwrap();
    }

    let response: RotatorResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &RotatorQueryMsg::Value)
        .unwrap();

    assert_eq!(
        "9476,9477,9478,9479,9480,9481,9482,9483,9484,9485",
        response.value
    );

    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &RotatorExecMsg::Clear,
        &[],
    )
    .unwrap();

    let response: RotatorResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &RotatorQueryMsg::Value)
        .unwrap();

    assert_eq!("0,0,0,0,0,0,0,0,0,0", response.value);
}
