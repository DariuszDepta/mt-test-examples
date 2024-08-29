#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::msg::{CounterActionMsg, CounterInitMsg, CounterQuery, CounterResponse};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError};
use cw_storage_plus::Item;

const COUNTER: Item<u8> = Item::new("value");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: CounterInitMsg,
) -> Result<Response, StdError> {
    COUNTER.save(
        deps.storage,
        &match msg {
            CounterInitMsg::Zero => 0,
            CounterInitMsg::Set(value) => value,
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: CounterActionMsg,
) -> Result<Response, StdError> {
    COUNTER.update::<_, StdError>(deps.storage, |old_value| {
        Ok(match msg {
            CounterActionMsg::Inc => old_value.saturating_add(1),
            CounterActionMsg::Dec => old_value.saturating_sub(1),
            CounterActionMsg::Set(new_value) => new_value,
        })
    })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: CounterQuery) -> Result<Binary, StdError> {
    match msg {
        CounterQuery::Value => Ok(to_json_binary(&CounterResponse {
            value: COUNTER.may_load(deps.storage).unwrap().unwrap(),
        })?),
    }
}
