#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::msg::{AdderExecuteMsg, AdderInitMsg, AdderQueryMsg, AdderResponse};
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: AdderInitMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: AdderExecuteMsg,
) -> StdResult<Response> {
    let AdderExecuteMsg::Add(a, b) = msg;
    Ok(Response::new().set_data(to_json_binary(&AdderResponse { sum: a + b })?))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: AdderQueryMsg) -> StdResult<Binary> {
    Ok(Binary::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> StdResult<Response> {
    Ok(Response::default())
}
