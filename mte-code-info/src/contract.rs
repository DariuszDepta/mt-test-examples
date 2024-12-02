#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::msg::{NayelQueryMsg, NayelResponse};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw_storage_plus::Item;

const ADDRESS: Item<String> = Item::new("address");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    ADDRESS.save(deps.storage, &env.contract.address.to_string())?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: NayelQueryMsg) -> StdResult<Binary> {
    match msg {
        NayelQueryMsg::Address => Ok(to_json_binary(&NayelResponse {
            address: ADDRESS.load(deps.storage)?,
        })?),
    }
}
