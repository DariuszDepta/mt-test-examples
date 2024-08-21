#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

const COUNTER: Item<u8> = Item::new("counter");

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CounterQuery {
    Counter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    COUNTER.save(deps.storage, &1)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    if let Some(mut counter) = COUNTER.may_load(deps.storage)? {
        counter = counter.saturating_add(1);
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
