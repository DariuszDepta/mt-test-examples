use crate::contract;
use cosmwasm_std::Empty;
use cw_multi_test::{App, Executor};

#[test]
fn instantiate_should_work() {
    // prepare the chain simulator
    let mut app = App::default();

    // prepare address of the contract creator
    let creator_addr = app.api().addr_make("creator");

    // store the contract's code on the chain
    let code_id = app.store_code_with_creator(creator_addr, contract());

    // prepare address of the contract owner
    let owner_addr = app.api().addr_make("owner");

    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner_addr,
            &Empty {},
            &[],
            "introductory-contract",
            None,
        )
        .unwrap();

    assert_eq!(
        contract_addr.as_str(),
        "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2"
    );
}
