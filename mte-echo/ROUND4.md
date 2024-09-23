```shell
$ mted tx wasm store ./target/wasm32-unknown-unknown/release/mte_echo.wasm --from alice --chain-id mte --gas 10000000 -y
$ mted q wasm list-code
$ mted tx wasm instantiate 4 {} --label my-counter --no-admin --from alice --chain-id mte -y
$ mted q wasm list-contract-by-code 4
```

```text
contracts:
- cosmos1kj8q8g2pmhnagmfepp9jh9g2mda7gzd0m5zdq0s08ulvac8ck4dq5k7g90
pagination:
  next_key: null
  total: "0"
```

```shell
$ mted q wasm contract-state smart cosmos1kj8q8g2pmhnagmfepp9jh9g2mda7gzd0m5zdq0s08ulvac8ck4dq5k7g90 '"count"'
```

```shell
$ mted tx wasm execute cosmos1kj8q8g2pmhnagmfepp9jh9g2mda7gzd0m5zdq0s08ulvac8ck4dq5k7g90 '{"send":"cosmos17lrcp5h9q37pvms2gm7mnln6kqvugjdajrgen5"}' --from alice --chain-id mte -y
#                                        bob's address --------------------------------------------^
```

FAILED
