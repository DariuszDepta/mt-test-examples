MultiTest | Examples

# Nayel

Example smart contract written using pure CosmWasm libraries that explains how to use code info functionality.

## Operations

Store the contract on chain:

```shell
$ wasmd tx wasm store ./target/wasm32-unknown-unknown/release/nayel.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y | jq
```

Output:
```json
{
  "height": "0",
  "txhash": "CC887A6365931A786717CE38C51B85E597DF6D25ADA2783BD9E1225467510F9A",
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
$ wasmd query wasm list-code -o json | jq
```

Output:

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "wasm1fcpp2g69ke5fmpt8ysfavxuqdq34h5sjg0gnmp",
      "data_hash": "07D52F0315BCCBF342DE79FECDDEA6474E868A522ACB9784D6F08996A43A1DAE",
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

Instantiate a contract: 

```shell
$ wasmd tx wasm instantiate 1 {} --label nayel-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y | jq
```

Output: 
```json
{
  "height": "0",
  "txhash": "55A28CD2B4AE73B18A588592BB503F493E73E33C01A966C471EEBF5A096495A0",
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



```shell
$ wasmd query wasm list-contract-by-code 1 -o json | jq
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

Query the current address of the contract:

```shell
$ wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"address"' -o json | jq
```

```json
{
  "data": {
    "address": "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"
  }
}
```
