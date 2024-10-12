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

_Output:_
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

_Output:_
```text
/usr/local/bin/wasmd
```

Check the version of `wasmd`:

```shell
wasmd version
```

_Output:_
```text
0.53.0-20-g8b8bb7c9
```

Initialize the node with a moniker (name) and a specific chain ID:

```shell
wasmd init wte --chain-id=wte
```

_Output:_
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

_Output:_
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

_Output:_
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

_Output:_
```text
Genesis transaction written to "~/.wasmd/config/gentx/gentx-8adbede8821774f5868719c0d1b5e6f9db5d1f36.json"
```

Collect genesis transactions to finalize the genesis file:

```shell
wasmd genesis collect-gentxs
```

_Output:_
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

_Output:_
```text
// many lines
5:08PM INF finalized block block_app_hash=67045FE5DDB84D6294D11E15B46B86503C400E4AC82D910C62ADB32C228D56E5 height=3 module=state num_txs_res=0 num_val_updates=0
5:08PM INF executed block app_hash=67045FE5DDB84D6294D11E15B46B86503C400E4AC82D910C62ADB32C228D56E5 height=3 module=state
5:08PM INF committed state block_app_hash=F6B5740B8346EB883E557165F780D1A646A73AB4819EA0CEA941B330C003801A height=3 module=state
5:08PM INF indexed block events height=3 module=txindex
```

Works! 🚀


---

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

```text
- address: wasm1hrh5k5utg4u266ewg2cn6nnsud88yssrja64y9
  name: alice
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Ajwy4MioSr9wvWNAHICES9J5ns23jH3VSYEI3Emug6vH"}'
  type: local


**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

identify steel vague asthma slim cinnamon disorder describe mother neutral train unusual hope solar trumpet naive amused eye behave purse nerve better ring walnut
```

```text
- address: wasm1uew8hw9y5z03atcf4zcw3k67g5uk0w7t904p9a
  name: bob
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AsMC5pR1EK31sryYKm2+nHP5+m/z2plPNh/FEtTEVWP5"}'
  type: local


**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

mix rapid keen motion spoil medal detect helmet sphere way stable crop accuse muscle prefer bleak dignity elegant pact announce time festival load eye
```

```shell
wasmd tx wasm store ./target/wasm32-unknown-unknown/release/mte_echo.wasm --from alice --chain-id wte --gas 10000000 --keyring-backend=test -o json -y
```

```shell
wasmd q wasm list-code
```

```shell
wasmd tx wasm instantiate 1 {} --label my-echo-1 --no-admin --from alice --chain-id wte -y -o json --keyring-backend=test
wasmd tx wasm instantiate 2 {} --label my-echo-1 --no-admin --from alice --chain-id wte -y -o json --keyring-backend=test
```

```shell
wasmd q wasm list-contract-by-code 1
```

```text
contracts:
- wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d
pagination:
  next_key: null
  total: "0"
```

```text
contracts:
- wasm1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqr5j2ht
pagination:
  next_key: null
  total: "0"
```

```shell
wasmd q wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"count"'
wasmd q wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '"replies"'
```

```shell
wasmd q bank balances wasm1hrh5k5utg4u266ewg2cn6nnsud88yssrja64y9 
```

```text
balances:
- amount: "999750000000"
  denom: stake
pagination:
  total: "1"
```

```shell
wasmd q bank balances wasm1uew8hw9y5z03atcf4zcw3k67g5uk0w7t904p9a
```

```text
balances:
- amount: "1000000000000"
  denom: stake
pagination:
  total: "1"
```

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
