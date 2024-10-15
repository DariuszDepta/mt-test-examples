# WASMD

This document describes how to prepare a blockchain node based on `wasmd`,
ready to test CosmWasm smart contracts written in Rust.
On Linux.

## Installing `Go`

Check if you have (possible the latest) version of Go installed:

```shell
go version
```

Output:
> go version go1.23.2 linux/amd64

To install a fresh version of Go or to upgrade, follow these [instructions](https://go.dev/doc/install).

## Removing the old `wasmd` node

Make sure, there is no previous `wasmd` node configured. Remove all previously created configuration files: 

```shell
rm -rf ~/.wasmd
```

Remove the local clone of the previously installed `wasmd` (only if you did this previously).

```shell
rm -rf ~/wasmd
```

## Installing the `wasmd` node

Change current working directory to your home directory:

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
> go install -mod=readonly -tags "netgo,ledger" -ldflags '-X github.com/cosmos/cosmos-sdk/version.Name=wasm -X github.com/cosmos/cosmos-sdk/version.AppName=wasmd
> -X github.com/cosmos/cosmos-sdk/version.Version=0.53.0-20-g8b8bb7c9 -X github.com/cosmos/cosmos-sdk/version.Commit=8b8bb7c9809cfc10c3e942f730b3cddb3e7a977d
> -X github.com/CosmWasm/wasmd/app.Bech32Prefix=wasm -X "github.com/cosmos/cosmos-sdk/version.BuildTags=netgo,ledger"' -trimpath ./cmd/wasmd
> go: downloading github.com/cosmos/cosmos-sdk v0.50.10
> go: downloading cosmossdk.io/x/tx v0.13.5


Check if `wasmd` is installed and available:

```shell
which wasmd
```

Output:
```text
/usr/local/bin/wasmd
```

Check the version of the installed `wasmd`:

```shell
wasmd version
```

Output:
```text
0.53.0-21-g028261cb
```

Initialize the node with a moniker (name) and a specific chain ID.
The node used in all examples is named **wte** and has also the same ID.  

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

Add key pairs for **alice**, **bob**, **cecil** and **dave** accounts.
The outputs of the commands below will be slightly different on your machine, but it's normal.

```shell
wasmd keys add alice --keyring-backend=test
```

Output:
```text
- address: wasm1p092r7ywtawlqmymzdkpehmvrac9gfw32l3ppf
  name: alice
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AgtKHeqMjLgHmhqu6pE0Fgj0F5AOs07a9jweojZKqP8+"}'
  type: local


**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

side swamp spike bright acid sell march submit usual stick party frog toddler wrong junk concert swallow sport speed direct input visual gloom myth
```

```shell
wasmd keys add bob --keyring-backend=test
```

Output:
```text
- address: wasm1fgjj69v0sg737cp6jsjtzqqsjxj6hc35rdn7gs
  name: bob
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A31YTJeU6P0qAll7ZjXikYw7l6IkdpYjcGYCp2nbbHdE"}'
  type: local


**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

earn smooth rally ignore walk hero kit delay famous excess grape rare donkey addict all craft lady march rose crowd drastic worry wall today
```

```shell
wasmd keys add cecil --keyring-backend=test
```

Output:
```text
- address: wasm1tnjp5ct9r35xnpkmc7p9ud445dl3zelfru6rxy
  name: cecil
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"ArHqfRf3sc2wm0yL1HRPbpKRlIqmp8kJsJiKPwaoGH19"}'
  type: local


**Important** write this mnemonic phrase in a safe place.
It is the only way to recover your account if you ever forget your password.

pen unfold menu tourist measure miss speed party online idea reason food put wide uncle medal office bomb enforce problem resource train turn frequent
```

The last account is created using JSON output and the result is formatted using `jq` (just to show how to do it).

```shell
wasmd keys add dave --keyring-backend=test --output json | jq
```

```json
{
  "name": "dave",
  "type": "local",
  "address": "wasm1qgukmr2qtaw9hqhx2pmlsk7e9lg7xh6t6dm3re",
  "pubkey": "{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AubAQdRFVaAw1pxu6oxXKwl//f/JdxZqv8ri6VWNAChO\"}",
  "mnemonic": "return dog crowd ranch notice jeans spare expand fossil glow kidney acoustic cage alert cluster nose firm address enhance repair pair balcony broken manual"
}
```

Add genesis accounts with initial balances for **alice**, **bob**, **cecil** and **dave**:


```shell
wasmd genesis add-genesis-account alice "1000000000000stake" --keyring-backend=test
```

```shell
wasmd genesis add-genesis-account bob "1000000000000stake" --keyring-backend=test
```

```shell
wasmd genesis add-genesis-account cecil "1000000000000stake" --keyring-backend=test
```

```shell
wasmd genesis add-genesis-account dave "1000000000000stake" --keyring-backend=test
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

🚀 The node is now running! 

## All commands in one place

```shell
wasmd init wte --chain-id=wte
wasmd keys add alice --keyring-backend=test
wasmd keys add bob --keyring-backend=test
wasmd keys add cecil --keyring-backend=test
wasmd keys add dave --keyring-backend=test
wasmd genesis add-genesis-account alice "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account bob "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account cecil "1000000000000stake" --keyring-backend=test
wasmd genesis add-genesis-account dave "1000000000000stake" --keyring-backend=test
wasmd genesis gentx alice "250000000stake" --chain-id=wte --amount="250000000stake" --keyring-backend=test
wasmd genesis collect-gentxs
wasmd start
```

## Retrieving the keys

If you forget the names of your accounts and the addresses, you can retrieve them the following way (the node must be running):

```shell
wasmd keys list --keyring-backend=test
```

Output (this data is fake, you will get something else):
```text
- address: wasm13un986wwc4cg82pdyy9j47k3fad6gv96ycxgmr
  name: alice
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A/LQsvrterteDAX5Vfq49Yigo+AQgro4T1cj1fmpFhC4"}'
  type: local
- address: wasm1rq9qlzaewq7frdg7re0hgrchutuz8hp8admfec
  name: bob
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Amd6g+A8lsK6zertertBc0p6jQ5pNFYyfiA+95BAj17J"}'
  type: local
- address: wasm1f82rwx883zuvc4dvgegpmmapddcj2dalke0tre
  name: cecil
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Arr9T+yCE6Ib2EcjWbmRSretertxQmHNX4iGYsuFkikG"}'
  type: local
- address: wasm104zw9gj273mwrlxhc5fzg600xa90gj750nxfaw
  name: dave
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A2VKLeziU6aw78IjsVp1CeVPkh4ertertM6j5BuitjTZ"}'
  type: local
```
