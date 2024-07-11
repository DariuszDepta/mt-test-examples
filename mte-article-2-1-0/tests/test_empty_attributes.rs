use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, Event, MessageInfo, Response, StdError};
use cw_multi_test::{Contract, ContractWrapper};

fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    Ok(Response::default())
}

fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    Ok(Response::<Empty>::new()
        .add_attribute("city", "    ")
        .add_attribute("street", "")
        .add_event(
            Event::new("location")
                .add_attribute("longitude", "   ")
                .add_attribute("latitude", ""),
        ))
}

fn query(_deps: Deps, _env: Env, _msg: Empty) -> Result<Binary, StdError> {
    Ok(Binary::default())
}

fn contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
}

#[test]
fn empty_string_attribute_should_work() {
    use cw_multi_test::{App, Executor};

    // prepare the blockchain
    let mut app = App::default();

    // prepare sender address
    let sender = app.api().addr_make("sender");

    // store the contract's code
    let code_id = app.store_code(contract());

    // instantiate the contract
    let contract_addr = app
        .instantiate_contract(code_id, sender.clone(), &Empty {}, &[], "attributed", None)
        .unwrap();

    // execute message on the contract, this returns response
    // with attributes having empty string values
    assert!(app
        .execute_contract(sender, contract_addr, &Empty {}, &[])
        .is_ok());
}
