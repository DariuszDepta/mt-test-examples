#[test]
fn default_block_should_work() {
    use cw_multi_test::App;

    // create the default chain simulator
    let app = App::default();

    // get the initial block properties
    let block = app.block_info();

    // default block height is 12345
    assert_eq!(12345, block.height);

    // default block timestamp is Wed Oct 23 2019 02:23:39 GMT+0000
    assert_eq!(1571797419879305533, block.time.nanos());

    // default chain identifier is "cosmos-testnet-14002"
    assert_eq!("cosmos-testnet-14002", block.chain_id);
}

#[test]
fn setting_initial_block_should_work() {
    use cosmwasm_std::{BlockInfo, Timestamp};
    use cw_multi_test::App;

    // create the default chain simulator
    let mut app = App::default();

    // create and use a custom block
    app.set_block(BlockInfo {
        height: 1,
        time: Timestamp::from_seconds(1723627489),
        chain_id: "starship-testnet".to_string(),
    });

    // get the custom block properties
    let block = app.block_info();

    // now the block height is 1
    assert_eq!(1, block.height);

    // now the block timestamp is Wed Aug 14 2024 09:24:49 GMT+0000
    assert_eq!(1723627489, block.time.seconds());

    // now the chain identifier is "starship-testnet"
    assert_eq!("starship-testnet", block.chain_id);
}

