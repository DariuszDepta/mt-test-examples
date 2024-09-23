use crate::msg::{EchoQuery, EchoResponse, ExecMessage, InitMessage};
use cosmwasm_std::{
    to_json_binary, Binary, CustomMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    SubMsgResponse, SubMsgResult,
};
use cw_storage_plus::Item;

const MSG_RESPONSES_COUNTER: Item<u64> = Item::new("msg-responses-counter");

pub fn instantiate<C>(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMessage<C>,
) -> StdResult<Response<C>>
where
    C: CustomMsg + 'static,
{
    let mut response = Response::new();
    if let Some(data) = msg.data {
        response = response.set_data(data.into_bytes());
    }
    if let Some(submessages) = msg.sub_msg {
        response = response.add_submessages(submessages);
    }
    Ok(response)
}

pub fn execute<C>(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecMessage<C>,
) -> StdResult<Response<C>>
where
    C: CustomMsg + 'static,
{
    let mut response = Response::new();
    if let Some(data) = msg.data {
        response = response.set_data(data.into_bytes());
    }
    if let Some(submessages) = msg.sub_msg {
        response = response.add_submessages(submessages);
    }
    Ok(response)
}

pub fn query(deps: Deps, _env: Env, msg: EchoQuery) -> StdResult<Binary> {
    match msg {
        EchoQuery::Count => Ok(to_json_binary(&EchoResponse {
            count: MSG_RESPONSES_COUNTER.may_load(deps.storage)?.unwrap(),
        })?),
    }
}

pub fn reply<C>(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response<C>>
where
    C: CustomMsg + 'static,
{
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
