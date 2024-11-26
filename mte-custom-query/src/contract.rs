#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::msg::{CounterExecMsg, CounterInitMsg, CounterQueryMsg, CounterResponse, ScadderQuery, ScadderResponse};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response, StdError};
use cw_storage_plus::Item;

const COUNTER: Item<u8> = Item::new("value");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ScadderQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: CounterInitMsg,
) -> Result<Response, StdError> {
    COUNTER.save(
        deps.storage,
        &match msg {
            CounterInitMsg::Zero => 0,
            CounterInitMsg::Set(new_value) => new_value,
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ScadderQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: CounterExecMsg,
) -> Result<Response, StdError> {
    COUNTER.update::<_, StdError>(deps.storage, |old_value| {
        Ok(match msg {
            CounterExecMsg::Inc => old_value.saturating_add(1),
            CounterExecMsg::Dec => old_value.saturating_sub(1),
            CounterExecMsg::Set(new_value) => new_value,
            CounterExecMsg::Add(x,y) => {
                let request = QueryRequest::Custom(ScadderQuery::Sum {value_x: x as u64, value_y: y as u64});
                if let Ok(sum) = deps.querier.query::<ScadderResponse>(&request) {
                    sum.value_sum.clamp(0,255) as u8
                } else {
                    255
                }
            }
        })
    })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ScadderQuery>, _env: Env, msg: CounterQueryMsg) -> Result<Binary, StdError> {
    match msg {
        CounterQueryMsg::Value => Ok(to_json_binary(&CounterResponse {
            value: COUNTER.may_load(deps.storage)?.unwrap(),
        })?),
    }
}
