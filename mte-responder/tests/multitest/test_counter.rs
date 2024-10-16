use cosmwasm_std::{BankMsg, Coin, Empty, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper, Executor, IntoAddr};
use mte_responder::msg::{ResponderReply, ResponderQueryMsg, ResponderCount, ResponderExecuteMsg};
use serde_json::Value;

const DENOM: &str = "stake";

fn replier_contract() -> Box<dyn Contract<Empty>> {
    Box::new(
        ContractWrapper::new_with_empty(
            mte_responder::contract::execute,
            mte_responder::contract::instantiate,
            mte_responder::contract::query,
        )
        .with_reply(mte_responder::contract::reply),
    )
}

fn coins(amount: u128) -> Vec<Coin> {
    vec![Coin {
        denom: DENOM.to_string(),
        amount: Uint128::new(amount),
    }]
}

fn assert_balance(amount: u128, coins: Vec<Coin>) {
    assert_eq!(1, coins.len());
    assert_eq!(amount, coins[0].amount.u128());
    assert_eq!(DENOM, coins[0].denom);
}

#[test]
fn instantiating_should_work() {
    let mut app = App::default();

    let code_id = app.store_code(replier_contract());

    let alice_addr = "alice".into_addr();

    let contract_addr = app
        .instantiate_contract(code_id, alice_addr, &Empty {}, &[], "replier-label", None)
        .unwrap();

    let res: ResponderCount = app
        .wrap()
        .query_wasm_smart(contract_addr, &ResponderQueryMsg::Count)
        .unwrap();

    assert_eq!(0, res.count);
}

#[test]
fn executing_should_work() {
    // Prepare addresses for Alice and Bob accounts.
    let alice_addr = "alice".into_addr();
    let bob_addr = "bob".into_addr();

    // Initialize the chain with initial balances for Alice and Bob.
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alice_addr, coins(999750000000))
            .unwrap();
        router
            .bank
            .init_balance(storage, &bob_addr, coins(1000000000000))
            .unwrap();
    });

    // Check the balance for Alice.
    assert_balance(
        999750000000,
        app.wrap().query_all_balances(&alice_addr).unwrap(),
    );

    // Check the balance for Bob.
    assert_balance(
        1000000000000,
        app.wrap().query_all_balances(&bob_addr).unwrap(),
    );

    // Alice stores the contract code on chain.
    let code_id = app.store_code_with_creator(alice_addr.clone(), replier_contract());

    // Alice instantiates the contract.
    let contract_addr = app
        .instantiate_contract(
            code_id,
            alice_addr.clone(),
            &Empty {},
            &[],
            "replier-label",
            None,
        )
        .unwrap();

    // Bob sends 100 tokens to the contract
    let msg = BankMsg::Send {
        to_address: contract_addr.to_string(),
        amount: coins(100),
    };
    app.execute(bob_addr.clone(), msg.into()).unwrap();

    // Check the balance for Bob.
    assert_balance(
        999999999900,
        app.wrap().query_all_balances(&bob_addr).unwrap(),
    );

    // Check the balance for the contract.
    assert_balance(100, app.wrap().query_all_balances(&contract_addr).unwrap());

    // Now the contract has 100 tokens. Let's execute the message on the contract and
    // ask the contract to transfer 10 coins to Bob. It will be done by returning bank message
    // which should be processed by chain and the reply should be "sent" back to the contract.
    let msg = ResponderExecuteMsg::BankSend(bob_addr.to_string(), 10, DENOM.to_string());
    app.execute_contract(alice_addr.clone(), contract_addr.clone(), &msg, &[])
        .unwrap();

    // Check the balance for Bob.
    assert_balance(
        999999999910,
        app.wrap().query_all_balances(&bob_addr).unwrap(),
    );

    // Check the balance for contract.
    assert_balance(90, app.wrap().query_all_balances(&contract_addr).unwrap());

    // There should be one reply message processed in the contract.
    let res: ResponderCount = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &ResponderQueryMsg::Replies)
        .unwrap();
    assert_eq!(1, res.count);

    let res: ResponderReply = app
        .wrap()
        .query_wasm_smart(contract_addr, &ResponderQueryMsg::Content)
        .unwrap();
    let content = res.content.clone();
    let json: Value = serde_json::from_str(content.as_str()).unwrap();
    let pretty = serde_json::to_string_pretty(&json).unwrap();
    let expected = r#"{
  "gas_used": 0,
  "id": 1,
  "payload": "",
  "result": {
    "ok": {
      "data": null,
      "events": [
        {
          "attributes": [
            {
              "key": "recipient",
              "value": "cosmwasm1sxmr0k8u6trd5c6eu6trzyapzux7090ykujmsng7pdx0m8k93n5sjrh9we"
            },
            {
              "key": "sender",
              "value": "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2"
            },
            {
              "key": "amount",
              "value": "10stake"
            }
          ],
          "type": "transfer"
        }
      ],
      "msg_responses": [
        {
          "type_url": "/cosmos.bank.v1beta1.MsgSendResponse",
          "value": ""
        }
      ]
    }
  }
}"#;
    assert_eq!(expected, pretty);
}
