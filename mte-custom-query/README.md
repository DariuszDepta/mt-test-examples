# Counter contract with custom query

## Testing on chain

Store the contract on chain:

```shell
sevdaysd tx wasm store scadder.wasm --from alice --chain-id sevdays --gas 10000000 -o json -y | jq
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
sevdaysd tx wasm instantiate 1 '"zero"' --label my-scadder-1 --no-admin --from alice --chain-id sevdays -o json -y | jq
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

