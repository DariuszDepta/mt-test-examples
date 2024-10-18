use crate::msg::{CounterInitMsg, CounterResponse};
use cosmwasm_std::{Response, StdResult};
use cw_storage_plus::Item;
use sylvia::contract;
use sylvia::types::{ExecCtx, InstantiateCtx, QueryCtx};

pub struct CounterContract {
    pub count: Item<u8>,
}

#[cfg_attr(not(feature = "library"), sylvia::entry_points)]
#[contract]
impl CounterContract {
    pub const fn new() -> Self {
        Self {
            count: Item::new("count"),
        }
    }

    #[sv::msg(instantiate)]
    fn zero(&self, ctx: InstantiateCtx, msg: CounterInitMsg) -> StdResult<Response> {
        match msg {
            CounterInitMsg::Zero => self.count.save(ctx.deps.storage, &0)?,
            CounterInitMsg::Set(value) => self.count.save(ctx.deps.storage, &value)?,
        }
        Ok(Response::new())
    }

    #[sv::msg(exec)]
    fn inc(&self, ctx: ExecCtx) -> StdResult<Response> {
        self.count
            .update(ctx.deps.storage, |count| -> StdResult<u8> {
                Ok(count.saturating_add(1))
            })?;
        Ok(Response::new())
    }

    #[sv::msg(exec)]
    fn dec(&self, ctx: ExecCtx) -> StdResult<Response> {
        self.count
            .update(ctx.deps.storage, |count| -> StdResult<u8> {
                Ok(count.saturating_sub(1))
            })?;
        Ok(Response::new())
    }

    #[sv::msg(exec)]
    fn set(&self, ctx: ExecCtx, value: u8) -> StdResult<Response> {
        self.count.save(ctx.deps.storage, &value)?;
        Ok(Response::new())
    }

    #[sv::msg(query)]
    fn value(&self, ctx: QueryCtx) -> StdResult<CounterResponse> {
        let value = self.count.load(ctx.deps.storage)?;
        Ok(CounterResponse { value })
    }
}
