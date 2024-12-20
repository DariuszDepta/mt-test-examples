use cosmwasm_std::Empty;
use scadder::msg::{CounterExecMsg, CounterInitMsg, CounterQueryMsg, CounterResponse, ScadderQuery};
use cw_multi_test::{custom_app, no_init, BasicApp, Contract, ContractWrapper, Executor, IntoAddr};

fn counter_contract() -> Box<dyn Contract<Empty, ScadderQuery>> {
    Box::new(ContractWrapper::new(
        scadder::contract::execute,
        scadder::contract::instantiate,
        scadder::contract::query,
    ))
}

#[test]
fn instantiating_with_zero_should_work() {
    let mut app: BasicApp<Empty, ScadderQuery> = custom_app::<Empty, ScadderQuery, _>(no_init);

    let code_id = app.store_code(counter_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner,
            &CounterInitMsg::Zero,
            &[],
            "counter-label",
            None,
        )
        .unwrap();

    let res: CounterResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &CounterQueryMsg::Value)
        .unwrap();

    assert_eq!(0, res.value);
}

#[test]
fn instantiating_with_value_should_work() {
    let mut app: BasicApp<Empty, ScadderQuery> = custom_app::<Empty, ScadderQuery, _>(no_init);

    let code_id = app.store_code(counter_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner,
            &CounterInitMsg::Set(12),
            &[],
            "counter-label",
            None,
        )
        .unwrap();

    let res: CounterResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &CounterQueryMsg::Value)
        .unwrap();

    assert_eq!(12, res.value);
}

#[test]
fn incrementing_should_work() {
    let mut app: BasicApp<Empty, ScadderQuery> = custom_app::<Empty, ScadderQuery, _>(no_init);

    let code_id = app.store_code(counter_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner.clone(),
            &CounterInitMsg::Zero,
            &[],
            "counter-contract",
            None,
        )
        .unwrap();

    app.execute_contract(owner, contract_addr.clone(), &CounterExecMsg::Inc, &[])
        .unwrap();

    let res: CounterResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &CounterQueryMsg::Value)
        .unwrap();

    assert_eq!(1, res.value);
}

#[test]
fn incrementing_should_stop_at_maximum() {
    let mut app: BasicApp<Empty, ScadderQuery> = custom_app::<Empty, ScadderQuery, _>(no_init);

    let code_id = app.store_code(counter_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner.clone(),
            &CounterInitMsg::Set(250),
            &[],
            "counter",
            None,
        )
        .unwrap();

    for _ in 1..=10 {
        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &CounterExecMsg::Inc,
            &[],
        )
        .unwrap();
    }

    let res: CounterResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &CounterQueryMsg::Value)
        .unwrap();

    assert_eq!(255, res.value);
}

#[test]
fn decrementing_should_work() {
    let mut app: BasicApp<Empty, ScadderQuery> = custom_app::<Empty, ScadderQuery, _>(no_init);

    let code_id = app.store_code(counter_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner.clone(),
            &CounterInitMsg::Set(126),
            &[],
            "counter-label",
            None,
        )
        .unwrap();

    app.execute_contract(owner, contract_addr.clone(), &CounterExecMsg::Dec, &[])
        .unwrap();

    let res: CounterResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &CounterQueryMsg::Value)
        .unwrap();

    assert_eq!(125, res.value);
}

#[test]
fn decrementing_should_stop_at_minimum() {
    let mut app: BasicApp<Empty, ScadderQuery> = custom_app::<Empty, ScadderQuery, _>(no_init);

    let code_id = app.store_code(counter_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner.clone(),
            &CounterInitMsg::Set(5),
            &[],
            "counter-label",
            None,
        )
        .unwrap();

    for _ in 1..=10 {
        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &CounterExecMsg::Dec,
            &[],
        )
        .unwrap();
    }

    let res: CounterResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &CounterQueryMsg::Value)
        .unwrap();

    assert_eq!(0, res.value);
}

#[test]
fn setting_value_should_work() {
    let mut app: BasicApp<Empty, ScadderQuery> = custom_app::<Empty, ScadderQuery, _>(no_init);

    let code_id = app.store_code(counter_contract());

    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner.clone(),
            &CounterInitMsg::Set(5),
            &[],
            "counter-label",
            None,
        )
        .unwrap();

    app.execute_contract(owner, contract_addr.clone(), &CounterExecMsg::Set(126), &[])
        .unwrap();

    let res: CounterResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &CounterQueryMsg::Value)
        .unwrap();

    assert_eq!(126, res.value);
}
