use counter::contract::sv::{ContractExecMsg, ContractQueryMsg};
use counter::msg::CounterInitMsg;
use sylvia::cw_schema::write_api;

fn main() {
    write_api! {
        instantiate: CounterInitMsg,
        execute: ContractExecMsg,
        query: ContractQueryMsg,
    }
}
