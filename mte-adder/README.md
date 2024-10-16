MultiTest | Examples

# Adder

Simple smart contract that adds two numbers.

## Features
- Initializing the sum of the last operation by evaluating the `instantiate` entry-point.
- Adding two integers by evaluating the `execute` entry-point.
- Adding two integers by evaluating the `query` entry-point.
- Querying the sum of the last `execute` operation by evaluating `query` entrypoint. 

## Compile the contract

Compile the contract to WASM binary:

```shell
cargo build --release --target wasm32-unknown-unknown --lib
```

_Output:_
```text
// many lines before
Compiling serde-json-wasm v1.0.1
Compiling cosmwasm-std v2.1.4
Compiling cosmwasm-schema v2.1.4
Compiling cw-storage-plus v2.0.0
Compiling mte-adder v0.1.0
 Finished `release` profile [optimized] target(s) in 14.50s
```

The contract's binary is here: `./target/wasm32-unknown-unknown/release/mte_adder.wasm`

## Interactions

To prepare the fresh instance of the `wasmd` node follow [these instructions](../WASMD.md). Start the `wasmd` node.
 
Store the **adder** contract on chain: 

```shell
wasmd tx wasm store ./target/wasm32-unknown-unknown/release/mte_adder.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

_Output:_
```json
{
  "height": "0",
  "txhash": "3AAB9086C36062521F9C0BD9C72C24F64E0BF34095E35CDBC662007E32F7CBF9",
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
      "creator": "wasm1r87s7mprs8fsh4fdcc9m5dpng7mq09mnf6u2tl",
      "data_hash": "2F29145BBF5E5333D9F3FCBBAAD9E70D02C6F24B63AA9C1C50EDC34FD547DC4E",
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

Check what contains the data in the transaction: 

```shell
wasmd query tx 3AAB9086C36062521F9C0BD9C72C24F64E0BF34095E35CDBC662007E32F7CBF9 -o json | jq '.data' | xxd -r -ps | decode_raw
```

_Output:_
```text
2 {
· 1: (38 bytes) "/cosmwasm.wasm.v1.MsgStoreCodeResponse"
· 2 {
· · 1: 1
· · 2: (32 bytes) 2f29145bbf5e5333d9f3fcbbaad9e70d02c6f24b63aa9c1c50edc34fd547dc4e
· }
}
```

Instantiate the contract with zero:

```shell
wasmd tx wasm instantiate 1 '"zero"' --label my-adder-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

_Output:_
```json
{
  "height": "0",
  "txhash": "5D67B5473F836F59232A83472EC91B0F445270AA4BA7A04A5238E618AD2D24BC",
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

## Adding numbers

Let's add two integers evaluating `execute` entry-point:

```shell
wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"add":[111,22]}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
```

_Output:_
```json
{
  "height": "0",
  "txhash": "033917BCE4F59DDA86AC6BC5DAA6174C020C899552A09DDA2DF18A27F4C80503",
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

Check what the resulting data is in the transaction:

```shell
wasmd query tx 033917BCE4F59DDA86AC6BC5DAA6174C020C899552A09DDA2DF18A27F4C80503 -o json | jq '.data' | xxd -r -ps | decode_raw
```

_Output:_
```text
2 {
· 1: (44 bytes) "/cosmwasm.wasm.v1.MsgExecuteContractResponse"
· 2 {
· · 1: (11 bytes) '{"sum":133}'
· }
}
```

We can also ask the contract to add numbers without any transaction:

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"add":[333,22]}' -o json | jq
```

_Output:_
```json
{
  "data": {
    "sum": 355
  }
}
```
