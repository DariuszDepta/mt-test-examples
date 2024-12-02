use crate::defs::{Value, Values};
use crate::msg::{RotatorExecMsg, RotatorQueryMsg, RotatorResponse};
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
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: RotatorExecMsg,
) -> StdResult<Response> {
    match msg {
        RotatorExecMsg::Clear => {
            VALUES
                .access(&mut CwStorage(deps.storage))
                .set(&Values::default())?;
        }
        RotatorExecMsg::Value(value, decimals) => {
            let mut cw_storage = CwStorage(deps.storage);
            let mut values = VALUES
                .access(&cw_storage)
                .get()?
                .unwrap_or(Values::default());
            values.append(Value::new(value, decimals));
            VALUES.access(&mut cw_storage).set(&values)?;
        }
    }
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
