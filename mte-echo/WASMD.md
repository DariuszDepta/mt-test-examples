# WASMD

Running `replyer` contract on `wasmd` chain.
> (TODO) rename this project to `mte-replyer`, it is more adequate name for what the contract does

## Preparing the `wasmd` node

Make sure, there is no previous `wasmd` node configured:

```shell
rm -rf ~/.wasmd
```

Change current working directory to home:

```shell
cd ~
```

Clone the `wasmd` repository:

```shell
git clone https://github.com/CosmWasm/wasmd.git  
```

Change current working directory to `wasmd`:

```shell
cd wasmd
```

Install `wasmd`:

```shell
make install
```

Output:
```text
go install -mod=readonly -tags "netgo,ledger" -ldflags '-X github.com/cosmos/cosmos-sdk/version.Name=wasm -X github.com/cosmos/cosmos-sdk/version.AppName=wasmd
-X github.com/cosmos/cosmos-sdk/version.Version=0.53.0-20-g8b8bb7c9 -X github.com/cosmos/cosmos-sdk/version.Commit=8b8bb7c9809cfc10c3e942f730b3cddb3e7a977d
 -X github.com/CosmWasm/wasmd/app.Bech32Prefix=wasm -X "github.com/cosmos/cosmos-sdk/version.BuildTags=netgo,ledger"' -trimpath ./cmd/wasmd
go: downloading github.com/cosmos/cosmos-sdk v0.50.10
go: downloading cosmossdk.io/x/tx v0.13.5
```

Check if `wasmd` is installed and available:

```shell
which wasmd
```

Output:
```text
/usr/local/bin/wasmd
```

Check the version of `wasmd`:

```shell
wasmd version
```

Output:
```text
0.53.0-20-g8b8bb7c9
```

Initialize the node with a moniker (name) and a specific chain ID:

```shell
wasmd init wte --chain-id=wte
```

Output:
```text
{
 "moniker": "wte",
 "chain_id": "wte",
 "node_id": "8adbede8821774f5868719c0d1b5e6f9db5d1f36",
 "gentxs_dir": ""
  
 // many lines follow
}
```

Add key pairs for **alice** and **bob** accounts:

```shell
wasmd keys add alice --keyring-backend=test
```

Output:
```text
- address: wasm1luhze876rg7t03wzhu2uvs4c2ynkfr0gwgd4sf
  name: alice
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AmiZBEviMr2jXEceH2/QzJBTYLUnpbEHC1QpAU8DLCsF"}'
  type: local


**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

pride car mask mercy start once ribbon immense sauce used giggle vacuum barrel quick page one cart wear enlist flavor race resource tribe sport
```

```shell
wasmd keys add bob --keyring-backend=test
```

Output:
```text
- address: wasm19fuangk4gq63387qjxsnjr0c0dkkrru54f0eam
  name: bob
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A/TSQkoX9UfqD2uMXEjpB6X4QvWIb6FJE1dx+ayLque2"}'
  type: local


**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

hunt retire result pact maze bulb wink menu sail forward edit palace review dawn gift drip ignore relief kid day expose panther ticket law
```

Add genesis accounts with initial balances for **alice** and **bob**:


```shell
wasmd genesis add-genesis-account alice "1000000000000stake" --keyring-backend=test
```

```shell
wasmd genesis add-genesis-account bob "1000000000000stake" --keyring-backend=test
```

Create a genesis transaction for the **alice** account, making **alice** a validator:

```shell
wasmd genesis gentx alice "250000000stake" --chain-id=wte --amount="250000000stake" --keyring-backend=test
```

Output:
```text
Genesis transaction written to "~/.wasmd/config/gentx/gentx-8adbede8821774f5868719c0d1b5e6f9db5d1f36.json"
```

Collect genesis transactions to finalize the genesis file:

```shell
wasmd genesis collect-gentxs
```

Output:
```text
{
 "moniker": "wte",
 "chain_id": "wte",
 "node_id": "8adbede8821774f5868719c0d1b5e6f9db5d1f36",
 "gentxs_dir": "~/.wasmd/config/gentx",
   
 // many lines follow
}
```

Start the node:

```shell
wasmd start
```

Output:
```text
// many lines
5:08PM INF finalized block block_app_hash=67045FE5DDB84D6294D11E15B46B86503C400E4AC82D910C62ADB32C228D56E5 height=3 module=state num_txs_res=0 num_val_updates=0
5:08PM INF executed block app_hash=67045FE5DDB84D6294D11E15B46B86503C400E4AC82D910C62ADB32C228D56E5 height=3 module=state
5:08PM INF committed state block_app_hash=F6B5740B8346EB883E557165F780D1A646A73AB4819EA0CEA941B330C003801A height=3 module=state
5:08PM INF indexed block events height=3 module=txindex
```

Works! 🚀

### Summary

All the commands summarized in one place:

```shell
wasmd init wte --chain-id=wte
wasmd keys add alice --keyring-backend=test
wasmd keys add bob --keyring-backend=test
wasmd genesis add-genesis-account alice "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account bob "1000000000000stake" --keyring-backend=test
wasmd genesis gentx alice "250000000stake" --chain-id=wte --amount="250000000stake" --keyring-backend=test
wasmd genesis collect-gentxs
wasmd start
```

## Compiling the contract

```shell
cargo build --release --target wasm32-unknown-unknown --lib
```

Output:
```text
    // many lines

   Compiling cosmwasm-std v2.1.4
   Compiling cosmwasm-schema v2.1.4
   Compiling cw-storage-plus v2.0.0
   Compiling mte-echo v0.1.0 (~/mte-echo)
    Finished `release` profile [optimized] target(s) in 13.96s
```

## Putting the contract on the chain

Store the contract on chain (**alice** is the code owner):

```shell
wasmd tx wasm store ./target/wasm32-unknown-unknown/release/mte_echo.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y
```

Output:
```text
{"height":"0","txhash":"56F8FD29C552D2D594C05D8F694747E41804AE35C9E5E8CFC7EFD0EACCBC0774","codespace":"","code":0,"data":"","raw_log":"","logs":[],"info":"","gas_wanted":"0","gas_used":"0","tx":null,"timestamp":"","events":[]}
```

Check if the contract code was stored:

```shell
wasmd query wasm list-code
```

Output:
```text
code_infos:
- code_id: "1"
  creator: wasm1luhze876rg7t03wzhu2uvs4c2ynkfr0gwgd4sf
  data_hash: 51DD73E52F219D353228721B72E77E5364D413899707174F3538BD4F2D0A8E00
  instantiate_permission:
    addresses: []
    permission: Everybody
pagination:
  next_key: null
  total: "0"
```

Instantiate the contract (**alice** is the creator of the instance):

```shell
wasmd tx wasm instantiate 1 {} --label my-replyer-1 --no-admin --from alice --chain-id wte --keyring-backend=test -o json -y
```

Output:
```text
{"height":"0","txhash":"6ACA83C6D928484984B76D75A94B18E87F6C5BC2F2E1CFFDAD8371E06B5B4DDA","codespace":"","code":0,"data":"","raw_log":"","logs":[],"info":"","gas_wanted":"0","gas_used":"0","tx":null,"timestamp":"","events":[]}
```

Check the list of instantiated contracts:

```shell
wasmd query wasm list-contract-by-code 1
```

Output:
```text
contracts:
- wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d
pagination:
  next_key: null
  total: "0"
```

## Interacting with the contract

### Querying the contract

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"count"' -o json | jq
```

Output:
```json
{
  "data": {
    "count": 0
  }
}
```

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"replies"' -o json | jq
```

Output:
```json
{
  "data": {
    "count": 0
  }
}
```

```shell
wasmd query wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"content"' -o json | jq
```

Output:
```text
ERR failure when running app err="rpc error: code = Unknown desc = Error calling the VM: Error executing Wasm: Wasmer runtime error: RuntimeError: Aborted: panicked at src/contract.rs:63:69:\ncalled `Option::unwrap()` on a `None` value: wasmvm error: unknown request"
```


Query the balances of **alice**:

```shell
wasmd query bank balances wasm1luhze876rg7t03wzhu2uvs4c2ynkfr0gwgd4sf -o json | jq 
```

```json
{
  "balances": [
    {
      "denom": "stake",
      "amount": "999750000000"
    }
  ],
  "pagination": {
    "total": "1"
  }
}
```

Query the balances of **bob**:

```shell
wasmd query bank balances wasm19fuangk4gq63387qjxsnjr0c0dkkrru54f0eam -o json | jq
```

```json
{
  "balances": [
    {
      "denom": "stake",
      "amount": "1000000000000"
    }
  ],
  "pagination": {
    "total": "1"
  }
}
```

## Sending tokens to contract

Check the contract address:

```shell
wasmd query wasm list-contract-by-code 1 -o json | jq '.contracts' 
```

Output:
```json
[
  "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"
]
```

Send 100stake from **bob** to 

```shell
wasmd tx bank send wasm1hrh5k5utg4u266ewg2cn6nnsud88yssrja64y9 wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d 100stake -o json --chain-id=wte --keyring-backend=test -y
wasmd tx bank send wasm1hrh5k5utg4u266ewg2cn6nnsud88yssrja64y9 wasm1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqr5j2ht 100stake -o json --chain-id=wte --keyring-backend=test -y
```

```shell
wasmd q bank balances wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d
wasmd q bank balances wasm1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqr5j2ht
```

```text
balances:
- amount: "100"
  denom: stake
pagination:
  total: "1"
```

```shell
wasmd tx wasm execute wasm1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqr5j2ht '{"send":"wasm1uew8hw9y5z03atcf4zcw3k67g5uk0w7t904p9a"}' --from alice --chain-id wte --keyring-backend=test -y -o json
# contract address ---^                                                 # bob's address --------^
```

```shell
wasmd q wasm contract-state smart wasm1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqr5j2ht '"replies"'
```

```text
data:
  content: '{"id":1,"payload":"","gas_used":0,"result":{"ok":{"events":[],"data":null,"msg_responses":[{"type_url":"/cosmos.bank.v1beta1.MsgSendResponse","value":""}]}}}'
```
