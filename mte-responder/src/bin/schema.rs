use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use mte_responder::msg::{ResponderExecuteMsg, ResponderQueryMsg};

fn main() {
    write_api! {
        instantiate: Empty,
        execute: ResponderExecuteMsg,
        query: ResponderQueryMsg,
    }
}
