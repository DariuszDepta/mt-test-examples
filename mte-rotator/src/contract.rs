use crate::defs::Values;
use crate::msg::{RotatorQueryMsg, RotatorResponse};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};
use cw_storey::containers::Item;
use cw_storey::CwStorage;

const VALUES: Item<Values> = Item::new(0);

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    VALUES
        .access(&mut CwStorage(deps.storage))
        .set(&Values::default())?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: RotatorQueryMsg) -> StdResult<Binary> {
    match msg {
        RotatorQueryMsg::Value => Ok(to_json_binary(&RotatorResponse {
            value: VALUES
                .access(&CwStorage(deps.storage))
                .get()?
                .unwrap_or(Values::default())
                .to_json(),
        })?),
    }
}
