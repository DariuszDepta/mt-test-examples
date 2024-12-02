use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum RotatorExecMsg {
    Clear,
    Value(u64, u8),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum RotatorQueryMsg {
    #[returns(RotatorResponse)]
    Value,
}

#[cw_serde]
pub struct RotatorResponse {
    pub value: String,
}
