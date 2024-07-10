#[test]
fn staking_unstaking_should_work() {
    use cosmwasm_std::testing::mock_env;
    use cosmwasm_std::{coin, Decimal, StakingMsg, Validator};
    use cw_multi_test::{AppBuilder, Executor, IntoBech32, StakingInfo};

    /// Denominator of the staking token.
    const BONDED_DENOM: &str = "stake";
    // Time between unbonding and receiving tokens back (in seconds).
    const UNBONDING_TIME: u64 = 60;
    // Amount of tokens to be (re)delegated
    const DELEGATION_AMOUNT: u128 = 100;
    // Initial amount of tokens for delegator.
    const INITIAL_AMOUNT: u128 = 1000;
    // Amount of tokens after delegation.
    const FEWER_AMOUNT: u128 = INITIAL_AMOUNT - DELEGATION_AMOUNT;

    let delegator_addr = "delegator".into_bech32();
    let validator_addr = "valoper".into_bech32();

    let valoper = Validator::new(
        validator_addr.to_string(),
        Decimal::percent(10),
        Decimal::percent(90),
        Decimal::percent(1),
    );

    // prepare the blockchain configuration
    let block = mock_env().block;
    let mut app = AppBuilder::default().build(|router, api, storage| {
        // set initial balance for the delegator
        router
            .bank
            .init_balance(
                storage,
                &delegator_addr,
                vec![coin(INITIAL_AMOUNT, BONDED_DENOM)],
            )
            .unwrap();
        // setup staking parameters
        router
            .staking
            .setup(
                storage,
                StakingInfo {
                    bonded_denom: BONDED_DENOM.to_string(),
                    unbonding_time: UNBONDING_TIME,
                    apr: Decimal::percent(10),
                },
            )
            .unwrap();
        // add a validator
        router
            .staking
            .add_validator(api, storage, &block, valoper)
            .unwrap();
    });

    // delegate tokens to validator
    app.execute(
        delegator_addr.clone(),
        StakingMsg::Delegate {
            validator: validator_addr.to_string(),
            amount: coin(DELEGATION_AMOUNT, BONDED_DENOM),
        }
        .into(),
    )
    .unwrap();

    // delegation works immediately, so delegator should have now fewer tokens
    let delegator_balance = app
        .wrap()
        .query_balance(delegator_addr.clone(), BONDED_DENOM)
        .unwrap();
    assert_eq!(FEWER_AMOUNT, delegator_balance.amount.u128());

    // validator should have now DELEGATION_AMOUNT of tokens assigned
    let delegation = app
        .wrap()
        .query_delegation(delegator_addr.clone(), validator_addr.clone())
        .unwrap()
        .unwrap();
    assert_eq!(DELEGATION_AMOUNT, delegation.amount.amount.u128());

    // now, undelegate all bonded tokens
    app.execute(
        delegator_addr.clone(),
        StakingMsg::Undelegate {
            validator: validator_addr.to_string(),
            amount: coin(DELEGATION_AMOUNT, BONDED_DENOM),
        }
        .into(),
    )
    .unwrap();

    // unbonding works with timeout, so tokens will be given back after unbonding time;
    // while we do not change the block height and time, delegator should still have fewer tokens
    let delegator_balance = app
        .wrap()
        .query_balance(delegator_addr.clone(), BONDED_DENOM)
        .unwrap();
    assert_eq!(FEWER_AMOUNT, delegator_balance.amount.u128());

    // now we update the block height and time that is shorter than the unbonding time
    app.update_block(|block| {
        block.height += 1;
        block.time = block.time.plus_seconds(UNBONDING_TIME - 1);
    });

    // delegator should still have fewer tokens
    let delegator_balance = app
        .wrap()
        .query_balance(delegator_addr.clone(), BONDED_DENOM)
        .unwrap();
    assert_eq!(FEWER_AMOUNT, delegator_balance.amount.u128());

    // now we update the block height and time again, so unbonding time is reached
    app.update_block(|block| {
        block.height += 1;
        block.time = block.time.plus_seconds(1);
    });

    // delegator should now have the initial amount of tokens back
    let delegator_balance = app
        .wrap()
        .query_balance(delegator_addr.clone(), BONDED_DENOM)
        .unwrap();
    assert_eq!(INITIAL_AMOUNT, delegator_balance.amount.u128());

    // there should be no more delegations
    let delegation = app
        .wrap()
        .query_delegation(delegator_addr, validator_addr)
        .unwrap();
    assert_eq!(None, delegation);
}

#[test]
fn advancing_block_should_work() {
    use cw_multi_test::App;

    let mut app = App::default();

    // let's check the default block height and time
    let default_height = app.block_info().height;
    let default_time = app.block_info().time.nanos();
    assert_eq!(12345, default_height);
    // timestamp of value 1571797419879305533 represents
    // Wed Oct 23 2019 02:23:39 GMT+0000
    assert_eq!(1571797419879305533, default_time);

    // let's advance the block height with 1, and time with 1 minute
    app.update_block(|block| {
        block.height += 1;
        block.time = block.time.plus_minutes(1);
    });

    // let's retrieve the new block's height and time
    let new_height = app.block_info().height;
    let new_time = app.block_info().time.nanos();

    // the block height should be advances with 1
    assert_eq!(new_height, default_height + 1);
    // the block time should be advances with 1 minute
    // 1 minute = 60 s = 60_000_000_000 ns
    assert_eq!(new_time, default_time + 60_000_000_000);

    // new height is:
    assert_eq!(12346, new_height);
    // new timestamp of value 1571797479879305533 represents
    // Wed Oct 23 2019 02:24:39 GMT+0000
    // so 1 minute later than the default value
    assert_eq!(1571797479879305533, new_time);
}
