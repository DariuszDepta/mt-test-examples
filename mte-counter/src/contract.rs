use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
};
use cw_storage_plus::Item;

const COUNTER: Item<u8> = Item::new("counter");

#[cw_serde]
pub enum CounterMsg {
    Increment,
    Decrement,
}

#[cw_serde]
pub enum CounterQuery {
    Counter,
}

#[cw_serde]
pub struct CounterResponse {
    pub value: u8,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    COUNTER.save(deps.storage, &0)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: CounterMsg,
) -> Result<Response, StdError> {
    if let Some(mut counter) = COUNTER.may_load(deps.storage)? {
        match msg {
            CounterMsg::Increment => {
                counter = counter.saturating_add(1);
            }
            CounterMsg::Decrement => {
                counter = counter.saturating_sub(1);
            }
        }
        COUNTER.save(deps.storage, &counter)?;
    }
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: CounterQuery) -> Result<Binary, StdError> {
    match msg {
        CounterQuery::Counter => Ok(to_json_binary(&CounterResponse {
            value: COUNTER.may_load(deps.storage).unwrap().unwrap(),
        })?),
    }
}
