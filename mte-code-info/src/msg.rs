use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum NayelQueryMsg {
    #[returns(NayelResponse)]
    Address,
}

#[cw_serde]
pub struct NayelResponse {
    pub address: String,
}
