use cosmwasm_std::{Coin, CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum VirtualStakeCustomQuery {
    VirtualStake(VirtualStakeQuery),
}

impl CustomQuery for VirtualStakeCustomQuery {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum VirtualStakeQuery {
    BondStatus { contract: String },
    SlashRatio {},
}

pub struct BondStatusResponse {
    pub cap: Coin,
    pub delegated: Coin,
}

pub struct SlashRatioResponse {
    pub slash_fraction_downtime: String,
    pub slash_fraction_double_sign: String,
}
