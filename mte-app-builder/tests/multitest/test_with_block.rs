#[test]
fn with_block_should_work() {
    use cosmwasm_std::{BlockInfo, Timestamp};
    use cw_multi_test::{no_init, AppBuilder};

    // create the chain builder
    let builder = AppBuilder::default();

    // prepare the custom block
    let block = BlockInfo {
        height: 21,
        time: Timestamp::from_nanos(1_571_797_419_879_305_544),
        chain_id: "milky-way-testnet".to_string(),
    };

    // build the chain initialized with the custom block
    let app = builder.with_block(block).build(no_init);

    // get the current block properties
    let block = app.block_info();

    // now the block height is 21
    assert_eq!(21, block.height);

    // now the block timestamp is Wed Oct 23 2019 02:23:39 GMT+0000
    assert_eq!(1_571_797_419_879_305_544, block.time.nanos());

    // now the chain identifier is "milky-way-testnet"
    assert_eq!("milky-way-testnet", block.chain_id);
}
