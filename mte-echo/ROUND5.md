```shell
mted tx wasm store ./target/wasm32-unknown-unknown/release/mte_echo.wasm --from alice --chain-id mte --gas 10000000 -y
mted q wasm list-code
mted tx wasm instantiate 5 {} --label my-counter --no-admin --from alice --chain-id mte -y
mted q wasm list-contract-by-code 5
```

```text
contracts:
- cosmos1sthrn5ep8ls5vzz8f9gp89khhmedahhdqd244dh9uqzk3hx2pzrst6fhxa
pagination:
  next_key: null
  total: "0"
```

```shell
mted q wasm contract-state smart cosmos1sthrn5ep8ls5vzz8f9gp89khhmedahhdqd244dh9uqzk3hx2pzrst6fhxa '"count"'
mted q wasm contract-state smart cosmos1sthrn5ep8ls5vzz8f9gp89khhmedahhdqd244dh9uqzk3hx2pzrst6fhxa '"replies"'
```

```shell
mted tx wasm execute cosmos1sthrn5ep8ls5vzz8f9gp89khhmedahhdqd244dh9uqzk3hx2pzrst6fhxa '{"send":"cosmos17lrcp5h9q37pvms2gm7mnln6kqvugjdajrgen5"}' --from alice --chain-id mte -y
                                                                         # bob's address --------^
```

```shell
mted q wasm contract-state smart cosmos1sthrn5ep8ls5vzz8f9gp89khhmedahhdqd244dh9uqzk3hx2pzrst6fhxa '"content"'
```

```text
data:
  content: '{"id":1,"payload":"","gas_used":0,"result":{"error":"codespace: sdk, code: 5"}}'
```

It looks like there is insufficient funds to process the request.
Should the contract have these funds?

```shell
mted tx bank send cosmos1cy0x6ax4a3je5nsf266xvldcf5qdq7rsps35a0 cosmos17lrcp5h9q37pvms2gm7mnln6kqvugjdajrgen5 10token
```

```shell
mted tx bank send cosmos1cy0x6ax4a3je5nsf266xvldcf5qdq7rsps35a0 cosmos1sthrn5ep8ls5vzz8f9gp89khhmedahhdqd244dh9uqzk3hx2pzrst6fhxa 10token
```

```shell
mted query bank balances cosmos1sthrn5ep8ls5vzz8f9gp89khhmedahhdqd244dh9uqzk3hx2pzrst6fhxa
```

```shell
mted q wasm contract-state smart cosmos1sthrn5ep8ls5vzz8f9gp89khhmedahhdqd244dh9uqzk3hx2pzrst6fhxa '"content"'
```

```text
data:
  content: '{"id":1,"payload":"","gas_used":0,"result":{"ok":{"events":[],"data":null,"msg_responses":[]}}}'
```
