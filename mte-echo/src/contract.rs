#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::msg::{EchoContent, EchoQuery, EchoResponse, ExecMessage};
use cosmwasm_std::{
    to_json_binary, to_json_string, BankMsg, Binary, Coin, Deps, DepsMut, Empty, Env, MessageInfo,
    Reply, ReplyOn, Response, StdError, StdResult, SubMsg, SubMsgResponse, SubMsgResult, Uint128,
};
use cw_storage_plus::Item;

const MSG_RESPONSES_COUNTER: Item<u64> = Item::new("msg-responses-counter");
const REPLY_COUNTER: Item<u64> = Item::new("reply-counter");
const RESULT: Item<Reply> = Item::new("reply");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    MSG_RESPONSES_COUNTER.save(deps.storage, &0)?;
    REPLY_COUNTER.save(deps.storage, &0)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecMessage,
) -> StdResult<Response> {
    let mut response = Response::default();
    match msg {
        ExecMessage::Send(addr) => {
            let b = BankMsg::Send {
                to_address: addr,
                amount: vec![Coin::new(Uint128::new(10), "stake")],
            };
            response = response.add_submessage(SubMsg {
                id: 1,
                payload: Default::default(),
                msg: b.into(),
                gas_limit: None,
                reply_on: ReplyOn::Always,
            });
        }
    }
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: EchoQuery) -> StdResult<Binary> {
    match msg {
        EchoQuery::Count => Ok(to_json_binary(&EchoResponse {
            count: MSG_RESPONSES_COUNTER.may_load(deps.storage)?.unwrap(),
        })?),
        EchoQuery::Replies => Ok(to_json_binary(&EchoResponse {
            count: REPLY_COUNTER.may_load(deps.storage)?.unwrap(),
        })?),
        EchoQuery::Content => Ok(to_json_binary(&EchoContent {
            content: to_json_string(&RESULT.may_load(deps.storage)?.unwrap()).unwrap(),
        })?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    REPLY_COUNTER.update::<_, StdError>(deps.storage, |value| Ok(value + 1))?;
    RESULT.save(deps.storage, &msg)?;
    #[allow(deprecated)]
    if let Reply {
        id: _,
        result:
            SubMsgResult::Ok(SubMsgResponse {
                events: _,
                data: _,
                msg_responses,
            }),
        ..
    } = msg
    {
        let count: u64 = msg_responses.len() as u64;
        MSG_RESPONSES_COUNTER.save(deps.storage, &count)?;
    }
    Ok(Response::default())
}
