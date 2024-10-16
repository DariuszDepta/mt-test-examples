#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::msg::{AdderExecuteMsg, AdderInitMsg, AdderQueryMsg, AdderResponse};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_storage_plus::Item;

/// The sum stored in the contract's state.
const SUM: Item<u128> = Item::new("sum");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: AdderInitMsg,
) -> StdResult<Response> {
    let sum = match msg {
        AdderInitMsg::Zero => 0,
        AdderInitMsg::Add(a, b) => add(a, b),
    };
    SUM.save(deps.storage, &sum)?;
    response(sum)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: AdderExecuteMsg,
) -> StdResult<Response> {
    let AdderExecuteMsg::Add(a, b) = msg;
    let sum = add(a, b);
    SUM.save(deps.storage, &sum)?;
    response(sum)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: AdderQueryMsg) -> StdResult<Binary> {
    match msg {
        AdderQueryMsg::Sum => {
            let sum = SUM.load(deps.storage)?;
            response_bin(sum)
        }
        AdderQueryMsg::Add(a, b) => {
            let sum = add(a, b);
            response_bin(sum)
        }
    }
}

/// Utility function for adding two numbers.
fn add(a: u128, b: u128) -> u128 {
    a.saturating_add(b)
}

/// Utility function for creating a response containing provided sum.
fn response(sum: u128) -> StdResult<Response> {
    Ok(Response::new().set_data(to_json_binary(&AdderResponse { sum })?))
}

/// Utility function for creating a binary containing provided sum.
fn response_bin(sum: u128) -> StdResult<Binary> {
    to_json_binary(&AdderResponse { sum })
}
