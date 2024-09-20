use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum CounterInitMsg {
    Zero,
    Set(u8),
}

#[cw_serde]
pub enum CounterActionMsg {
    Inc,
    Dec,
    Set(u8),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum CounterQuery {
    #[returns(CounterResponse)]
    Value,
}

#[cw_serde]
pub struct CounterResponse {
    pub value: u8,
}
