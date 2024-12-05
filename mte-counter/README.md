MultiTest | Examples

# Counter

Example **counter** smart contract written using pure CosmWasm libraries.

## Compiling

```shell
cargo +stable build
```

## Testing

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
wasmd tx wasm instantiate 1 '"zero"' --label my-counter-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

```shell
wasmd query wasm list-contract-by-code 1 -o json | jq
```

Output:
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
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"value"' -o json | jq
```

Output:
```json
{
  "data": {
    "value": 0
  }
}
```

Increment value:

```shell
wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"inc"' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
```

Query the current value of the counter contract (should be 1):

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"value"' -o json | jq
```

Output:
```json
{
  "data": {
    "value": 1
  }
}
```

Set the value of the counter to 53:

```shell
wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"set":53}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
```

Query the current value of the counter contract (should be 53):

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"value"' -o json | jq
```


docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/optimizer:0.16.1
