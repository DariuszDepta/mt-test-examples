# Counter contract with custom query

## Testing on chain

Store the contract on chain:

```shell
sevdaysd tx wasm store scadder.wasm --from alice --chain-id sevdays --gas 10000000 -y -o json | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "C6D5A33D0EA67D8A20FA2F57C19A75BB9DD413AE8A7977F3E096B22E2E09FA6C",
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

Check if the contract code was stored on chain.

```shell
sevdaysd query wasm list-code -o json | jq
```

Output:

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "cosmos1w0jpjm0xm0yh8yetzghy3mkpdp6dsjrka68ekw",
      "data_hash": "ECC84DAE8370C0B6A4A80A30D77FE7AC5C03030269FD47F83057AD3695D53533",
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

Instantiate a contract with initial value set to 0. 

```shell
sevdaysd tx wasm instantiate 1 '"zero"' --label my-scadder-1 --no-admin --from alice --chain-id sevdays -y -o json | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "E909709C8D291A3111BC18A883FA639528E480147A61A5C77D3EAD99C1EAF32D",
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

List all instantiated contracts from code id = 1.

```shell
sevdaysd query wasm list-contract-by-code 1 -o json | jq
```

Output:

```json
{
  "contracts": [
    "cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

Query the current value of the counter contract (should be 0):

```shell
sevdaysd query wasm contract-state smart cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '"value"' -o json | jq
```

Output:

```json
{
  "data": {
    "value": 0
  }
}
```

Set the value of the counter to 53:

```shell
sevdaysd tx wasm execute cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '{"set":53}' --from alice --chain-id sevdays -y -o json | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "B3D19BE180FE2799349D6FD349A930D2DCB764A41420F303AD754C413D0144E4",
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

Query the current value of the counter contract (should be 53):

```shell
sevdaysd query wasm contract-state smart cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '"value"' -o json | jq
```

Output:

```json
{
  "data": {
    "value": 53
  }
}
```

Increment the counter value by one:

```shell
sevdaysd tx wasm execute cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '"inc"' --from alice --chain-id sevdays -y -o json | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "126C1E2B95A0F76435114FD7500B2409D66BB7881E52C1E2F7F75D841B14E03F",
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

Query the current value of the counter contract (should be 54):

```shell
sevdaysd query wasm contract-state smart cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '"value"' -o json | jq
```

```json
{
  "data": {
    "value": 54
  }
}
```

Add two values and set as a counter's value:

```shell
sevdaysd tx wasm execute cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '{"add":[10,20]}' --from alice --chain-id sevdays -y -o json | jq
```

Output:

```json
{
  "height": "0",
  "txhash": "A9905287B3435E08FE324AED461A00D2686F4B815D8AB344E4256F277E1C9465",
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

Query the current value of the counter contract:

```shell
sevdaysd query wasm contract-state smart cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '"value"' -o json | jq
```

Output:
```json
{
  "data": {
    "value": 255
  }
}
```

Ok, the query has failed...


Instantiate a contract with initial value set to 0 but from the second code id. 

```shell
sevdaysd tx wasm instantiate 2 '"zero"' --label my-scadder-1 --no-admin --from alice --chain-id sevdays -y -o json | jq
```

List all instantiated contracts from code id = 2.

```shell
sevdaysd query wasm list-contract-by-code 2 -o json | jq
```

```json
{
  "contracts": [
    "cosmos1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqez7la9"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

Query the current value of the counter contract:

```shell
sevdaysd query wasm contract-state smart cosmos1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqez7la9 '"value"' -o json | jq
```

```json
{
  "data": {
    "value": 0
  }
}
```

Add two values and set as a counter's value:

```shell
sevdaysd tx wasm execute cosmos1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqez7la9 '{"add":[10,20]}' --from alice --chain-id sevdays -y -o json | jq
```


```shell
sevdaysd query wasm contract-state smart cosmos1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqez7la9 '"text"' -o json | jq
```
