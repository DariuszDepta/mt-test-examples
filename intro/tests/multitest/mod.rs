mod test_execute;
mod test_instantiate;
mod test_query;

use cosmwasm_std::Empty;
use cw_multi_test::{Contract, ContractWrapper};

pub fn intro() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new_with_empty(
        intro::contract::execute,
        intro::contract::instantiate,
        intro::contract::query,
    ))
}
