use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Response, StdResult};
use cw_storage_plus::Item;
use sylvia::contract;
use sylvia::types::{ExecCtx, InstantiateCtx, QueryCtx};

#[cw_serde]
pub struct CountResponse {
    pub count: u64,
}

pub struct CounterContract {
    pub count: Item<u64>,
}

#[cfg_attr(not(feature = "library"), sylvia::entry_points)]
#[contract]
impl CounterContract {
    pub const fn new() -> Self {
        Self { count: Item::new("count") }
    }

    #[sv::msg(instantiate)]
    fn instantiate(&self, ctx: InstantiateCtx) -> StdResult<Response> {
        self.count.save(ctx.deps.storage, &0)?;
        Ok(Response::new())
    }

    #[sv::msg(exec)]
    fn increment(&self, ctx: ExecCtx) -> StdResult<Response> {
        self.count.update(ctx.deps.storage, |count| -> StdResult<u64> { Ok(count.saturating_add(1)) })?;
        Ok(Response::new())
    }

    #[sv::msg(query)]
    fn count(&self, ctx: QueryCtx) -> StdResult<CountResponse> {
        let count = self.count.load(ctx.deps.storage)?;
        Ok(CountResponse { count })
    }
}
