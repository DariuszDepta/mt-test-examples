mod multitest;

use cosmwasm_std::Empty;
use cw_multi_test::{Contract, ContractWrapper};
use intro::contract::{execute, instantiate, query};

pub fn contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
}
