MultiTest | Examples

# Counter

Counter smart contract written using Sylvia framework.

## Compiling

```shell
cargo +stable build
```

## Testing using `MultiTest`

```shell
cargo +stable test
```

```shell
cargo +stable nextest run
```

## Building WASM binary

```shell
cargo build --release --target wasm32-unknown-unknown --lib
```

## Building schema

```shell
cargo run --bin schema
```

## Testing on chain

Store the contract on chain:

```shell
wasmd tx wasm store ./target/wasm32-unknown-unknown/release/counter.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

Check if the contract code was stored on chain.

```shell
wasmd query wasm list-code -o json | jq
```

Instantiate a contract with initial value set to 0. 

```shell
wasmd tx wasm instantiate 1 '{"msg":{"zero":{}}}' --label my-counter-sv-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

Query all contracts instantiated based on code_id = 1:

```shell
wasmd query wasm list-contract-by-code 1 -o json | jq
```

_Output_:
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

Query the current value of the counter contract (should be 0):

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"value":{}}' -o json | jq
```

_Output_:
```json
{
  "data": {
    "value": 0
  }
}
```

Let's increment the value...

```shell
wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"inc":{}}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
``` 

...and check it (should be 1):

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"value":{}}' -o json | jq
```

_Output_:
```json
{
  "data": {
    "value": 1
  }
}
```

Let's set the counter value to 255:

```shell
wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"set":{"value":255}}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
```

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"value":{}}' -o json | jq
```

_Output_:
```json
{
  "data": {
    "value": 255
  }
}
```

Let's instantiate another contract with initial value set to 87. 

```shell
wasmd tx wasm instantiate 1 '{"msg":{"set":87}}' --label my-counter-sv-2 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

Query all contracts instantiated based on code_id = 1:

```shell
wasmd query wasm list-contract-by-code 1 -o json | jq
```

_Output:_
```json
{
  "contracts": [
    "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
    "wasm1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrss5maay"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

Let's check the value:

```shell
wasmd query wasm contract-state smart wasm1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrss5maay '{"value":{}}' -o json | jq
```

_Output_:
```json
{
  "data": {
    "value": 87
  }
}
```

---

```shell
wasmd query tx B7FE592E071CB42FC80D44D43BCCED483A1FF1A2B5F76C97ECD636D06C763D3A -o json | jq
```
