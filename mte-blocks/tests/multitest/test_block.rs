use cw_multi_test::App;

#[test]
fn default_block_should_work() {
    let app = App::default();
    let block = app.block_info();
    assert_eq!(1571797419879305533, block.time.nanos());
    assert_eq!(12345, block.height);
    assert_eq!("cosmos-testnet-14002", block.chain_id);
}
