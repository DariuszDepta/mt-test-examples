```shell
wasmd tx wasm store ./target/wasm32-unknown-unknown/release/mte_adder.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

```shell
wasmd query wasm list-code -o json | jq
```

```shell
wasmd tx wasm instantiate 2 '"reset"' --label my-adder-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```


```shell
wasmd query wasm list-contract-by-code 2 -o json | jq
```

Output:
```text
{
  "contracts": [
    "wasm1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqr5j2ht"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

```shell
wasmd query wasm contract-state smart wasm1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqr5j2ht '{"add":[1,2]}' -o json | jq
```


```shell
wasmd tx wasm execute wasm1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqr5j2ht '{"add":[1,2]}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
```

                                 



Query for specific transaction:

```shell
wasmd query tx transaction_hash -o json | jq
```


wasmd query tx 69F911A326DCB5E2B4CA8AD6DEE25FDA305BB300881AAB43B3A11F429CC6A80A -o json | jq
