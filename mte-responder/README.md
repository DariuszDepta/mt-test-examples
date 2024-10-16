MultiTest | Examples

# Responder

## Compile the contract

Compile the contract to WASM binary:

```shell
cargo build --release --target wasm32-unknown-unknown --lib
```

_Output:_
```text
// many lines before
Compiling cosmwasm-std v2.1.4
Compiling cosmwasm-schema v2.1.4
Compiling cw-storage-plus v2.0.0
Compiling mte-responder v0.1.0
 Finished `release` profile [optimized] target(s) in 14.60s
```

The contract's binary is here: `./target/wasm32-unknown-unknown/release/mte_responder.wasm`

## Interactions

To prepare the fresh instance of the `wasmd` node follow [these instructions](../WASMD.md). Start the `wasmd` node.
 
Store the **responder** contract on chain: 

```shell
wasmd tx wasm store ./target/wasm32-unknown-unknown/release/mte_responder.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

_Output:_
```json
{
  "height": "0",
  "txhash": "4AB7FE08078AA04E63D345DFA95BA66DBD89A2C93297EA9C74A4E2015A22F64D",
  "codespace": "",
  "code": 0,
  "data": "",
  "raw_log": "",
  "logs": [],
  "info": "",
  "gas_wanted": "0",
  "gas_used": "0",
  "tx": null,
  "timestamp": "",
  "events": []
}
```

Check if the contract's code was stored on chain: 

```shell
wasmd query wasm list-code -o json | jq
```

_Output:_
```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "wasm1wa2tcsyza8n8rp2ftq0720m23eff3y63gmr037",
      "data_hash": "DE643497275CDA139B51925BC2C61B33B1B797656F93F36BF00CE4CA50509439",
      "instantiate_permission": {
        "permission": "Everybody",
        "addresses": []
      }
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

Instantiate the contract, `Empty` initialization message, **alice** is the creator of the instance:

```shell
wasmd tx wasm instantiate 1 {} --label my-responder-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

_Output:_
```json
{
  "height": "0",
  "txhash": "237261AA329AD17A9FBF05DACF8F405D4EA0E66D151B4EDAA79882B563BF5084",
  "codespace": "",
  "code": 0,
  "data": "",
  "raw_log": "",
  "logs": [],
  "info": "",
  "gas_wanted": "0",
  "gas_used": "0",
  "tx": null,
  "timestamp": "",
  "events": []
}
```

Check the list of instantiated contracts:

```shell
wasmd query wasm list-contract-by-code 1 -o json | jq
```

_Output:_
```json
{
  "contracts": [
    "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

## Sending tokens to contract

Check the contract address:

```shell
wasmd query wasm list-contract-by-code 1 -o json | jq '.contracts[0]' 
```

_Output:_
```text
"wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"
```

Check the **alice** address:

```shell
wasmd keys list --keyring-backend=test --output=json | jq '.[] | select(.name == "alice").address'
```

_Output:_
```text
"wasm1wa2tcsyza8n8rp2ftq0720m23eff3y63gmr037"
```

**alice** sends 100 coins to contract:

```shell
wasmd tx bank send wasm1wa2tcsyza8n8rp2ftq0720m23eff3y63gmr037 wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d 100stake --chain-id=wte --keyring-backend=test -y -o json | jq
```

Let's check the contract balance:

```shell
wasmd query bank balances wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d -o json | jq
```

_Output_:
```json
{
  "balances": [
    {
      "denom": "stake",
      "amount": "100"
    }
  ],
  "pagination": {
    "total": "1"
  }
}
```

## Sending tokens to users

Now the contract instantiated by **alice** has some coins. **alice** may send some coins to other users using the **responder** contract.

**alice** wants to send 10 coins to **bob**. Let's check the **bob**'s address:

```shell
wasmd keys list --keyring-backend=test --output=json | jq '.[] | select(.name == "bob").address'
```

_Output_:
```text
"wasm1n49nhq3q8avhx7ys86k7620pt7je8007neq3fk"
```

Let's check the **bob**'s balance:

```shell
wasmd query bank balances wasm1n49nhq3q8avhx7ys86k7620pt7je8007neq3fk -o json | jq
```

_Output_:
```json
{
  "balances": [
    {
      "denom": "stake",
      "amount": "1000000000000"
    }
  ],
  "pagination": {
    "total": "1"
  }
}
```

**alice** sends 10 coins to **bob**:

```shell
wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"bank_send":["wasm1n49nhq3q8avhx7ys86k7620pt7je8007neq3fk",10,"stake"]}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
# contract address ---^                                                              bob's address ---^                                  amount ---^   ^--- denom       ^--- alice is calling the contract
```

Let's check the **bob**'s balance again:

```shell
wasmd query bank balances wasm1n49nhq3q8avhx7ys86k7620pt7je8007neq3fk -o json | jq '.balances'
```

_Output_:
```json
[
  {
    "denom": "stake",
    "amount": "1000000000010"
  }
]
```

Let's check the contract's balance:

```shell
wasmd query bank balances wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d -o json | jq '.balances'
```

_Output_:
```json
[
  {
    "denom": "stake",
    "amount": "90"
  }
]
```

## Verifying reply from `BankMsg::Send`

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"content"' -o json | jq
```

_Output_:
```json
{
  "data": {
    "content": "{\"id\":1,\"payload\":\"\",\"gas_used\":0,\"result\":{\"ok\":{\"events\":[],\"data\":null,\"msg_responses\":[{\"type_url\":\"/cosmos.bank.v1beta1.MsgSendResponse\",\"value\":\"\"}]}}}"
  }
}
```

## Burning coins

The contract has now 90 coins. **alice** wants to burn them all!

```shell
wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"bank_burn":[90,"stake"]}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
# contract address ---^                                                                   amount ---^   ^--- denom        ^--- alice is calling the contract
```

Let's check the contract's balance after burning:

```shell
wasmd query bank balances wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d -o json | jq '.balances'
```

_Output_:
```json
[]
```

## Verifying reply from `BankMsg::Burn`

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"content"' -o json | jq
```

_Output_:
```json
{
  "data": {
    "content": "{\"id\":2,\"payload\":\"\",\"gas_used\":0,\"result\":{\"ok\":{\"events\":[],\"data\":null,\"msg_responses\":[]}}}"
  }
}
```

> Hmmm, `msg_responses` is empty for `BankMsg::Burn`. 
