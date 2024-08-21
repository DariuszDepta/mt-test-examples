use counter::contract::sv::mt::{CodeId, CounterContractProxy};
use sylvia::cw_multi_test::IntoAddr;
use sylvia::multitest::App;

#[test]
fn instantiate() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let owner ="owner".into_addr();
    let contract = code_id.instantiate().call(&owner).unwrap();

    assert_eq!(0, contract.count().unwrap().count);

    contract.increment().call(&owner).unwrap();
    assert_eq!(1, contract.count().unwrap().count);
}
