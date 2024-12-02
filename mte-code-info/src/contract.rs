#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::msg::{NayelQueryMsg, NayelResponse};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw_storage_plus::Item;

const ADDRESS: Item<String> = Item::new("address");
const REASON: Item<String> = Item::new("reason");
const CODE_ID: Item<u64> = Item::new("code-id");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    let contract_addr = env.contract.address;
    ADDRESS.save(deps.storage, &contract_addr.to_string())?;
    match deps.querier.query_wasm_contract_info(contract_addr) {
        Ok(contract_info_response) => CODE_ID.save(deps.storage,&contract_info_response.code_id)?,
        Err(reason) => REASON.save(deps.storage, &reason.to_string())?,
    }
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
            value: ADDRESS.load(deps.storage)?,
        })?),
        NayelQueryMsg::Reason => Ok(to_json_binary(&NayelResponse {
            value: REASON.load(deps.storage)?,
        })?),
        NayelQueryMsg::CodeId => Ok(to_json_binary(&NayelResponse {
            value: format!("{}", CODE_ID.load(deps.storage)?),
        })?),
    }
}
