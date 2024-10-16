use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum AdderInitMsg {
    Zero,
    Add(u128, u128),
}

#[cw_serde]
pub enum AdderExecuteMsg {
    Add(u128, u128),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum AdderQueryMsg {
    #[returns(AdderResponse)]
    Sum,
    #[returns(AdderResponse)]
    Add(u128, u128),
}

#[cw_serde]
pub struct AdderResponse {
    pub sum: u128,
}
