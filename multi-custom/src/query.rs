use cosmwasm_std::Coin;

pub enum VirtualStakeCustomQuery {
    VirtualStake(VirtualStakeQuery),
}

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
