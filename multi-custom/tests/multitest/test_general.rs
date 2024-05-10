use cosmwasm_std::Empty;
use cw_multi_test::{custom_app, no_init, App, Contract, ContractWrapper, Executor};
use multi_custom::converter_contract;
use multi_custom::msg::VirtualStakeCustomMsg;
use multi_custom::virtual_staking_contract;

fn converter_contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new(
        converter_contract::execute,
        converter_contract::instantiate,
        converter_contract::query,
    ))
}

fn converter_contract_empty() -> Box<dyn Contract<VirtualStakeCustomMsg>> {
    Box::new(ContractWrapper::new_with_empty(
        converter_contract::execute,
        converter_contract::instantiate,
        converter_contract::query,
    ))
}

fn virtual_staking_contract() -> Box<dyn Contract<VirtualStakeCustomMsg>> {
    Box::new(ContractWrapper::new(
        virtual_staking_contract::execute,
        virtual_staking_contract::instantiate,
        virtual_staking_contract::query,
    ))
}

#[test]
fn instantiate_converter_contract_should_work() {
    // prepare the simulator
    let mut app = App::default();

    // prepare address of the contract creator
    let creator_addr = app.api().addr_make("creator");

    // store the contract's code on the chain
    let code_id = app.store_code_with_creator(creator_addr, converter_contract());

    // prepare address of the contract owner
    let owner_addr = app.api().addr_make("owner");

    // invoke the `instantiate` entry-point of the contract
    let contract_addr = app.instantiate_contract(code_id, owner_addr, &Empty {}, &[], "converter-contract", None).unwrap();

    // contract is instantiated, we have a valid contract address
    assert_eq!(contract_addr.as_str(), "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2");
}

#[test]
fn instantiate_virtual_staking_contract_should_work() {
    // prepare the simulator
    let mut app = custom_app::<VirtualStakeCustomMsg, Empty, _>(no_init);

    // prepare address of the contract creator
    let creator_addr = app.api().addr_make("creator");

    // store the contract's code on the chain
    let code_id = app.store_code_with_creator(creator_addr, virtual_staking_contract());

    // prepare address of the contract owner
    let owner_addr = app.api().addr_make("owner");

    // invoke the `instantiate` entry-point of the contract
    let contract_addr = app.instantiate_contract(code_id, owner_addr, &Empty {}, &[], "converter-contract", None).unwrap();

    // contract is instantiated, we have a valid contract address
    assert_eq!(contract_addr.as_str(), "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2");
}

#[test]
fn instantiate_both_should_work() {
    // prepare the simulator
    let mut app = custom_app::<VirtualStakeCustomMsg, Empty, _>(no_init);

    // prepare address of the contract creator
    let creator_addr = app.api().addr_make("creator");

    // store the contract's code on the chain
    let _code_id_1 = app.store_code_with_creator(creator_addr.clone(), converter_contract_empty());

    // store the contract's code on the chain
    let _code_id_2 = app.store_code_with_creator(creator_addr, virtual_staking_contract());
}
