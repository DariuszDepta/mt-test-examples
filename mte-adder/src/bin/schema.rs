use cosmwasm_schema::write_api;
use mte_adder::msg::{AdderExecuteMsg, AdderInitMsg, AdderQueryMsg};

fn main() {
    write_api! {
        instantiate: AdderInitMsg,
        execute: AdderExecuteMsg,
        query: AdderQueryMsg,
    }
}
