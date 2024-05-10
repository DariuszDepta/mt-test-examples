#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::msg::VirtualStakeCustomMsg;
use crate::query::VirtualStakeCustomQuery;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(_deps: DepsMut<VirtualStakeCustomQuery>, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response<VirtualStakeCustomMsg>> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(_deps: DepsMut<VirtualStakeCustomQuery>, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response<VirtualStakeCustomMsg>> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps<VirtualStakeCustomQuery>, _env: Env, _msg: Empty) -> StdResult<Binary> {
    Ok(to_json_binary(&Empty {}).unwrap())
}
