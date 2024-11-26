use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
pub enum CounterInitMsg {
    Zero,
    Set(u8),
}

#[cw_serde]
pub enum CounterExecMsg {
    Inc,
    Dec,
    Set(u8),
    Add(u8, u8),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum CounterQueryMsg {
    #[returns(CounterResponse)]
    Value,
    #[returns(CounterTextResponse)]
    Text,
}

#[cw_serde]
pub struct CounterResponse {
    pub value: u8,
}

#[cw_serde]
pub struct CounterTextResponse {
    pub value: String,
}

#[cw_serde]
pub enum ScadderQuery {
    Sum { value_x: u64, value_y: u64 },
}

impl CustomQuery for ScadderQuery {}

#[cw_serde]
pub struct ScadderResponse {
    pub value_sum: u64,
}
