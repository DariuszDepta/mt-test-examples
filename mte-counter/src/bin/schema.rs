use cosmwasm_schema::write_api;

use counter::msg::{CounterExecMsg, CounterInitMsg, CounterQueryMsg};

fn main() {
    write_api! {
        instantiate: CounterInitMsg,
        execute: CounterExecMsg,
        query: CounterQueryMsg,
    }
}
