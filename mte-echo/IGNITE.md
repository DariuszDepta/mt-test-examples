# Echo contract

Store contract on the chain:

```shell
$ mted tx wasm store ./target/wasm32-unknown-unknown/release/mte_echo.wasm --from alice --chain-id mte --gas 10000000 -y
```

```shell
$ mted query tx --type=hash 146666A682A0CAB7FBBA75A01E0D7B16677A58055008D74C48510614A5A9C4BC | yq 'del(.tx.body.messages[0].wasm_byte_code)'
# was an error

$ mted query tx --type=hash 99E18C31D7944705D5203BC937FDE3F922CBE0F4A23E817E2ADB92FF73241BEF | yq 'del(.tx.body.messages[0].wasm_byte_code)'
# was ok
```

Check if the contract was stored:

```shell
$ mted q wasm list-code
```

```text
code_infos:
- code_id: "1"
  creator: cosmos1cy0x6ax4a3je5nsf266xvldcf5qdq7rsps35a0
  data_hash: E3999E6E26B67D96B60E07B52AA5D7A7B92B18C90DA3DE9762A002490721CB3F
  instantiate_permission:
    addresses: []
    permission: Everybody
- code_id: "2"
  creator: cosmos1cy0x6ax4a3je5nsf266xvldcf5qdq7rsps35a0
  data_hash: B53E801C8E0F077F2593FB7D185BC8555DEC2390B4BE927425158F4889163A64
  instantiate_permission:
    addresses: []
    permission: Everybody
pagination:
  next_key: null
  total: "0"
```

```shell
$ mted tx wasm instantiate 2 {} --label my-counter --no-admin --from alice --chain-id mte -y
```

List contracts by code id:

```shell
$ mted q wasm list-contract-by-code 2
```

```text
contracts:
- cosmos1xr3rq8yvd7qplsw5yx90ftsr2zdhg4e9z60h5duusgxpv72hud3s493rn8
pagination:
  next_key: null
  total: "0"
```

Query the current value of the contract:

```shell
$ mted q wasm contract-state smart cosmos1xr3rq8yvd7qplsw5yx90ftsr2zdhg4e9z60h5duusgxpv72hud3s493rn8 '"count"'
```

- 👤 alice's account address: cosmos1cy0x6ax4a3je5nsf266xvldcf5qdq7rsps35a0
- 👤 bob's account address: cosmos17lrcp5h9q37pvms2gm7mnln6kqvugjdajrgen5

```shell
$ mted query bank balances cosmos1cy0x6ax4a3je5nsf266xvldcf5qdq7rsps35a0
```

```text
balances:
- amount: "100000000"
  denom: stake
- amount: "20000"
  denom: token
pagination:
  total: "2"
```

```shell
$ mted query bank balances cosmos17lrcp5h9q37pvms2gm7mnln6kqvugjdajrgen5
```

```text
balances:
- amount: "100000000"
  denom: stake
- amount: "10000"
  denom: token
pagination:
  total: "2"
```

Let's execute message:

```shell
$ mted tx wasm execute cosmos1xr3rq8yvd7qplsw5yx90ftsr2zdhg4e9z60h5duusgxpv72hud3s493rn8 '{"send":"cosmos17lrcp5h9q37pvms2gm7mnln6kqvugjdajrgen5"}' --from alice --chain-id mte -y
#                                        bob's address --------------------------------------------^
```

```shell
$ mted query tx --type=hash CB8880687F8A3565384DA8E7A4E099921EB6C50DE54111A39E577553C940CDDB | yq 'del(.tx.body.messages[0].wasm_byte_code)'
# looks like ok
```

```shell
$ mted q wasm contract-state smart cosmos1xr3rq8yvd7qplsw5yx90ftsr2zdhg4e9z60h5duusgxpv72hud3s493rn8 '"count"'
```

# 3

```text
contracts:
- cosmos1qwlgtx52gsdu7dtp0cekka5zehdl0uj3fhp9acg325fvgs8jdzksx8qqr8
  pagination:
  next_key: null
  total: "0"
```

```shell
$ mted tx wasm execute cosmos1qwlgtx52gsdu7dtp0cekka5zehdl0uj3fhp9acg325fvgs8jdzksx8qqr8 '{"send":"cosmos17lrcp5h9q37pvms2gm7mnln6kqvugjdajrgen5"}' --from alice --chain-id mte -y
#                                        bob's address --------------------------------------------^
```

```shell
$ mted q wasm contract-state smart cosmos1qwlgtx52gsdu7dtp0cekka5zehdl0uj3fhp9acg325fvgs8jdzksx8qqr8 '"count"'
```

```text

```

```shell
$ mted query tx --type=hash 15D68EE3A83E96092EEC4E0D2B942C0191EBC6F0548145A5DC7B44EC0AC63456 | yq 'del(.tx.body.messages[0].wasm_byte_code)'
```

