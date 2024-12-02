use cosmwasm_std::Empty;
use cw_multi_test::{App, Contract, ContractWrapper, Executor, IntoAddr};
use rotator::msg::{RotatorQueryMsg, RotatorResponse};

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
