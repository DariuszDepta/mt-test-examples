use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum NayelQueryMsg {
    #[returns(NayelResponse)]
    Address,
    #[returns(NayelResponse)]
    Reason,
    #[returns(NayelResponse)]
    CodeId,
}

#[cw_serde]
pub struct NayelResponse {
    pub value: String,
}
