use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum ResponderExecuteMsg {
    /// Attributes for executing BankMsg::Send as a submessage: recipient address, amount, denom.
    BankSend(String, u128, String),
    /// Attributes for executing BankMsg::Burn as a submessage: amount, denom.
    BankBurn(u128, String),
    /// Attributes for executing Adder::Add as a submessage: a, b.
    AdderAdd(String, u128, u128),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum ResponderQueryMsg {
    #[returns(ResponderCount)]
    Count,
    #[returns(ResponderCount)]
    Replies,
    #[returns(ResponderReply)]
    Content,
}

#[cw_serde]
pub struct ResponderCount {
    pub count: u64,
}

#[cw_serde]
pub struct ResponderReply {
    pub content: String,
}

#[cw_serde]
pub enum AdderExecuteMsg {
    Add(u128, u128),
}
