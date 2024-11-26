use cosmwasm_schema::write_api;

use scadder::msg::{CounterExecMsg, CounterInitMsg, CounterQueryMsg};

fn main() {
    write_api! {
        instantiate: CounterInitMsg,
        execute: CounterExecMsg,
        query: CounterQueryMsg,
    }
}
