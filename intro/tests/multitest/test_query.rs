use crate::multitest::intro;
use cosmwasm_std::Empty;
use cw_multi_test::{App, Executor};

#[test]
fn query_should_work() {
    // prepare the simulator
    let mut app = App::default();

    // prepare address of the contract creator
    let creator_addr = app.api().addr_make("creator");

    // store the contract's code on the chain
    let code_id = app.store_code_with_creator(creator_addr, intro());

    // prepare address of the contract owner
    let owner_addr = app.api().addr_make("owner");

    // create an instance of the contract and save its address
    let contract_addr = app.instantiate_contract(code_id, owner_addr, &Empty {}, &[], "introductory-contract", None).unwrap();

    // invoke the `query` entry-point of the contract,
    // which should return an empty message
    let _: Empty = app.wrap().query_wasm_smart(contract_addr, &Empty {}).unwrap();
}
