use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CustomMsg, SubMsg};

#[cw_serde]
#[derive(Default)]
pub struct InitMessage<C>
where
    C: CustomMsg + 'static,
{
    /// Data to be returned after processing `instantiate` entry-point.
    pub data: Option<String>,
    /// Submessages to be returned from `instantiate` entry-point.
    pub sub_msg: Option<Vec<SubMsg<C>>>,
}

#[cw_serde]
#[derive(Default)]
pub struct ExecMessage<C>
where
    C: CustomMsg + 'static,
{
    /// Data to be returned after processing `execute` entry-point.
    pub data: Option<String>,
    /// Submessages to be returned from `execute` entry-point.
    pub sub_msg: Option<Vec<SubMsg<C>>>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum EchoQuery {
    #[returns(EchoResponse)]
    Count,
}

#[cw_serde]
pub struct EchoResponse {
    pub count: u64,
}
