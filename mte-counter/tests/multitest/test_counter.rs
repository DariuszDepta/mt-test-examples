use cosmwasm_std::Empty;
use counter::contract::{CounterQuery, CounterResponse};
use cw_multi_test::{App, Contract, ContractWrapper, Executor, IntoAddr};

fn counter_contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new_with_empty(
        counter::contract::execute,
        counter::contract::instantiate,
        counter::contract::query,
    ))
}

#[test]
fn instantiating_should_work() {
    let mut app = App::default();

    let code_id = app.store_code(counter_contract());
    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(code_id, owner.clone(), &Empty {}, &[], "counter", None)
        .unwrap();

    let res: CounterResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &CounterQuery::Counter)
        .unwrap();

    assert_eq!(1, res.value);
}

#[test]
fn incrementing_should_work() {
    let mut app = App::default();

    let code_id = app.store_code(counter_contract());
    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(code_id, owner.clone(), &Empty {}, &[], "counter", None)
        .unwrap();

    app.execute_contract(owner, contract_addr.clone(), &Empty {}, &[])
        .unwrap();

    let res: CounterResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &CounterQuery::Counter)
        .unwrap();

    assert_eq!(2, res.value);
}

#[test]
fn incrementing_should_stop_at_255() {
    let mut app = App::default();

    let code_id = app.store_code(counter_contract());
    let owner = "owner".into_addr();

    let contract_addr = app
        .instantiate_contract(code_id, owner.clone(), &Empty {}, &[], "counter", None)
        .unwrap();

    for _ in 0..300 {
        app.execute_contract(owner.clone(), contract_addr.clone(), &Empty {}, &[])
            .unwrap();
    }

    let res: CounterResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &CounterQuery::Counter)
        .unwrap();

    assert_eq!(255, res.value);
}
