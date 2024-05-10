use cosmwasm_std::Coin;

pub enum VirtualStakeCustomMsg {
    VirtualStake(VirtualStakeMsg),
}

pub enum VirtualStakeMsg {
    Bond { amount: Coin, validator: String },
    Unbond { amount: Coin, validator: String },
}
