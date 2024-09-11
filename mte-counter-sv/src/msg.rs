use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum CounterInitMsg {
    Zero,
    Set(u8),
}

#[cw_serde]
pub struct CounterResponse {
    pub value: u8,
}
