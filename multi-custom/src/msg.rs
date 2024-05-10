use cosmwasm_std::{Coin, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum VirtualStakeCustomMsg {
    VirtualStake(VirtualStakeMsg),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum VirtualStakeMsg {
    Bond { amount: Coin, validator: String },
    Unbond { amount: Coin, validator: String },
}

impl CustomMsg for VirtualStakeCustomMsg {}
