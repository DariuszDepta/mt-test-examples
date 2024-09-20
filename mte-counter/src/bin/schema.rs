use cosmwasm_schema::write_api;

use counter::msg::{CounterActionMsg, CounterInitMsg, CounterQuery};

fn main() {
    write_api! {
        instantiate: CounterInitMsg,
        execute: CounterActionMsg,
        query: CounterQuery,
    }
}
