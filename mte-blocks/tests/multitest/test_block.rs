use cw_multi_test::App;

#[test]
fn default_block_should_work() {
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
