## Adder

```shell
wasmd query wasm list-contract-by-code 1 -o json | jq
```

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

## Responder

```shell
wasmd query wasm list-contract-by-code 2 -o json | jq
```

```json
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
wasmd tx wasm execute wasm1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqr5j2ht '{"adder_add":["wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",11,22]}' --from alice --chain-id wte --keyring-backend=test -y -o json | jq
```

```shell
wasmd query tx A77A6B6412ABABEC88AA6E6729485C9E35586AB18D6093CCFD44787E9B289930 -o json | jq
```


```shell
wasmd query tx A77A6B6412ABABEC88AA6E6729485C9E35586AB18D6093CCFD44787E9B289930 -o json | jq '.data' | xxd -r -ps | decode_raw
wasmd query tx A77A6B6412ABABEC88AA6E6729485C9E35586AB18D6093CCFD44787E9B289930 -o json | jq '.data' | xxd -r -ps | decode_raw
```

Query the reply content: 

```shell
wasmd query wasm contract-state smart wasm1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqr5j2ht '"content"' -o json | jq
```

_Output_:
```json
{
  "data": {
    "content": "{\"id\":2,\"payload\":\"\",\"gas_used\":0,\"result\":{\"ok\":{\"events\":[{\"type\":\"execute\",\"attributes\":[{\"key\":\"_contract_address\",\"value\":\"wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d\"}]}],\"data\":\"Cgp7InN1bSI6MzN9\",\"msg_responses\":[{\"type_url\":\"/cosmwasm.wasm.v1.MsgExecuteContractResponse\",\"value\":\"Cgp7InN1bSI6MzN9\"}]}}}"
  }
}
```

```json
[{"type_url":"/cosmwasm.wasm.v1.MsgExecuteContractResponse","value":"Cgp7InN1bSI6MzN9"}]
```

```shell 
echo "CGP7INN1BSI6MZN9" | xxd -r -ps | decode_raw
```

122E0A2C2F636F736D7761736D2E7761736D2E76312E4D736745786563757465436F6E7472616374526573706F6E7365

