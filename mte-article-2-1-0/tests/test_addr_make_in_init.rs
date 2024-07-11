#[test]
fn initializing_balance_later_should_work() {
    use cosmwasm_std::{Coin, Uint128};
    use cw_multi_test::App;

    const USER: &str = "alice";

    let coin = Coin {
        denom: "LUNC".to_string(),
        amount: Uint128::new(10),
    };

    // create a chain
    let mut app = App::default();

    // create single user address used in subsequent operations
    let user_addr = app.api().addr_make(USER);

    // fund some tokens to user address during chain initialization
    app.init_modules(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &user_addr, vec![coin])
            .unwrap();
    });

    // use user address to query it's balances
    let balances = app.wrap().query_all_balances(user_addr).unwrap();

    assert_eq!("LUNC", balances[0].denom);
    assert_eq!(10, balances[0].amount.u128());
}

#[test]
fn initializing_balance_in_new_should_work() {
    use cosmwasm_std::{Coin, Uint128};
    use cw_multi_test::App;

    const USER: &str = "alice";

    let coin = Coin {
        denom: "LUNC".to_string(),
        amount: Uint128::new(10),
    };

    // create and initialize the chain in one operation
    let app = App::new(|router, api, storage| {
        router
            .bank
            .init_balance(storage, &api.addr_make(USER), vec![coin])
            .unwrap();
    });

    // query user's balances
    let balances = app
        .wrap()
        .query_all_balances(app.api().addr_make(USER))
        .unwrap();

    assert_eq!("LUNC", balances[0].denom);
    assert_eq!(10, balances[0].amount.u128());
}

#[test]
fn initializing_balance_in_app_builder_should_work() {
    use cosmwasm_std::{Coin, Uint128};
    use cw_multi_test::AppBuilder;
    use cw_multi_test::MockApiBech32m;

    const USER: &str = "alice";

    let coin = Coin {
        denom: "LUNC".to_string(),
        amount: Uint128::new(10),
    };

    // create and initialize the chain in one operation
    // use Bech32m format for user addresses
    // use "juno" prefix for the chain
    let app = AppBuilder::new()
        .with_api(MockApiBech32m::new("juno"))
        .build(|router, api, storage| {
            router
                .bank
                .init_balance(storage, &api.addr_make(USER), vec![coin])
                .unwrap();
        });

    let user_addr = app.api().addr_make(USER);

    // just to see how the user address looks like
    assert_eq!(
        "juno190vqdjtlpcq27xslcveglfmr4ynfwg7gmw86cnun4acakxrdd6gqg3ddu4",
        user_addr.as_str()
    );

    // query user's balances
    let balances = app.wrap().query_all_balances(user_addr).unwrap();

    assert_eq!("LUNC", balances[0].denom);
    assert_eq!(10, balances[0].amount.u128());
}

#[test]
fn initializing_balance_custom_app_should_work() {
    use cosmwasm_std::{Coin, CustomMsg, CustomQuery, Uint128};
    use cw_multi_test::{custom_app, BasicApp};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    const USER: &str = "alice";

    let coin = Coin {
        denom: "LUNC".to_string(),
        amount: Uint128::new(10),
    };

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
    #[serde(rename = "snake_case")]
    pub enum ExampleMsg {
        MsgEx,
    }
    impl CustomMsg for ExampleMsg {}

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
    #[serde(rename = "snake_case")]
    pub enum ExampleQuery {
        QueryEx,
    }
    impl CustomQuery for ExampleQuery {}

    let app: BasicApp<ExampleMsg, ExampleQuery> = custom_app(|router, api, storage| {
        router
            .bank
            .init_balance(storage, &api.addr_make(USER), vec![coin])
            .unwrap();
    });

    // query user's balances
    let balances = app
        .wrap()
        .query_all_balances(app.api().addr_make(USER))
        .unwrap();

    assert_eq!("LUNC", balances[0].denom);
    assert_eq!(10, balances[0].amount.u128());
}
