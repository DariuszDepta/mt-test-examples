# WASMD

Running `replyer` contract on `wasmd` chain.
> (TODO) rename this project to `mte-replyer`, it is more adequate name for what the contract does

## Prerequisites

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
