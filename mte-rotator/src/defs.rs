use cosmwasm_schema::cw_serde;

pub const MAX_VALUE_COUNT: usize = 10;

#[cw_serde]
#[derive(Default, Copy)]
pub struct Value {
    value: u64,
    decimals: u8,
}

impl Value {
    pub fn new(value: u64, decimals: u8) -> Self {
        Self { value, decimals }
    }
}

#[cw_serde]
#[derive(Default, Copy)]
pub struct Values([Value; MAX_VALUE_COUNT]);

impl Values {
    pub fn to_json(&self) -> String {
        self.0
            .iter()
            .map(|value| format!("{}", value.value))
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn append(&mut self, value: Value) {
        self.0.rotate_left(1);
        self.0[MAX_VALUE_COUNT - 1] = value;
    }
}
