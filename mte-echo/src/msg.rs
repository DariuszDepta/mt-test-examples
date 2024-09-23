use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecMessage {
    Send(String),
}

#[cw_serde]
pub enum EchoQuery {
    Count,
    Replies,
    Content,
}

#[cw_serde]
pub struct EchoResponse {
    pub count: u64,
}

#[cw_serde]
pub struct EchoContent {
    pub content: String,
}
