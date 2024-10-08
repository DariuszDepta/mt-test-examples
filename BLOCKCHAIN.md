# Scaffolding a  blockchain

[Go]: https://go.dev/doc/install
[IgniteCLI]: https://docs.ignite.com/welcome/install
[Ignite CLI Wasm App]: https://github.com/ignite/apps/tree/main/wasm

## Overview

This document describes the procedure of creating a blockchain on Linux operating system,
ready to run the examples prepared with MultiTest.

## Scaffolding a chain

### Prerequisities

#### Install the newest version of [Go]

Check the installed [Go] version:

```shell
$ go version
```

Output:

```
go version go1.23.1 linux/amd64
```

#### Install the newest version of [IgniteCLI]

Check the installed [IgniteCLI] version:

```shell
$ ignite version
```

Output:

```text
Ignite CLI version:             v28.5.2
Ignite CLI build date:          2024-09-11T12:54:28Z
Ignite CLI source hash:         72244a722e45593aa7caa207621e2fb82e8df0d8
Ignite CLI config version:      v1
Cosmos SDK version:             v0.50.9
Your OS:                        linux
Your arch:                      amd64
Your go version:                go version go1.23.1 linux/amd64
Your uname -a:                  x86_64 GNU/Linux
Your cwd:                       (hidden)
Is on Gitpod:                   false
```

### Scaffold a chain

Stay inside the directory where th project **mt-test-examples** was cloned.

Scaffold a new chain named `mte`:

```shell
$ ignite scaffold chain mte
````

Output:

```text
⭐️ Successfully created a new blockchain 'mte'.
👉 Get started with the following commands:

 % cd mte
 % ignite chain serve

Documentation: https://docs.ignite.com
```

### Run the chain

```shell
$ cd mte
```

```shell
$ ignite chain serve
```

Output:

```text
  Blockchain is running
  
  ✔ Added account alice with address cosmos1243r4wftmxufaxqh6lar4hgf38j8405kh2ptyd and mnemonic:
  gap injury stomach clap burst rebuild eye slender urban prize pitch march penalty fine valley afford ankle employ recycle library embark theme october resist
  
  ✔ Added account bob with address cosmos1u7cgk2k0cza2s8zfmwmk7sc7mq3y8vevfdwuez and mnemonic:
  pet symbol stumble pupil strike boat physical dream rocket talk hub save shock mind age illegal aim urban exact own sense below what measure
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: /home/user/.mte
  ⋆ App binary: /home/user/go/bin/mted
  
  Press the 'q' key to stop serve
```

Press `q` to stop the chain.

Output:

```text
  💿 Genesis state saved in /home/user/.ignite/local-chains/mte/exported_genesis.json
  
  𝓲 Stopped
  
  💬 Survey: https://bit.ly/3WZS2uS
```

### Test the chain

```shell
$ ignite chain serve
```

Output:

```text
  Blockchain is running
  
  👤 alice's account address: cosmos1243r4wftmxufaxqh6lar4hgf38j8405kh2ptyd
  👤 bob's account address: cosmos1u7cgk2k0cza2s8zfmwmk7sc7mq3y8vevfdwuez
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: /home/user/.mte
  ⋆ App binary: /home/user/go/bin/mted
  
  Press the 'q' key to stop serve
```

Open another terminal check the alice's balances: 

```shell
$ mted query bank balances cosmos1243r4wftmxufaxqh6lar4hgf38j8405kh2ptyd
```

Output:

```text
balances:
- amount: "100000000"
  denom: stake
- amount: "20000"
  denom: token
pagination:
  total: "2"
```

Now check the bob's balances:

```shell
$ mted query bank balances cosmos1u7cgk2k0cza2s8zfmwmk7sc7mq3y8vevfdwuez
```

Output:

```text
balances:
- amount: "100000000"
  denom: stake
- amount: "10000"
  denom: token
  pagination:
  total: "2"
```

Press `q` to stop the chain.

Output:

```text
  💿 Genesis state saved in /home/user/.ignite/local-chains/mte/exported_genesis.json
  
  𝓲 Stopped
  
  💬 Survey: https://bit.ly/3WZS2uS
```

## Adding a `wasm` module to the chain

We are basing on this tutorial: [Ignite CLI Wasm App].

Install Ignite CLI Wasm app:

```shell
$ ignite app install -g github.com/ignite/apps/wasm
```

Output:

```text
✔ Done loading apps
🎉 Installed github.com/ignite/apps/wasm
```

Add Wasm support

```shell
$ ignite wasm add
```

Output:

```text
create app/ante.go
modify app/app.go
modify app/app_config.go
modify app/ibc.go
create app/wasm.go
modify cmd/mted/cmd/commands.go

🎉 CosmWasm added (`[some path]/mte`).
```

Start the chain:

```shell
$ ignite chain serve
```

Output:

```text
  cannot build app:                                                                   
                                                                                      
  error while running command go build -o /home/user/go/bin/mted -mod readonly      
  -tags  -ldflags -X github.com/cosmos/cosmos-sdk/version.Name=Mte -X                 
  github.com/cosmos/cosmos-sdk/version.AppName=mted -X                                
  github.com/cosmos/cosmos-sdk/version.Version= -X                                    
  github.com/cosmos/cosmos-sdk/version.Commit=ae6413a442814fe8adb395ea7b3f8917cc7b87f6
  -X github.com/cosmos/cosmos-sdk/version.BuildTags= -X                               
  mte/cmd/mted/cmd.ChainID=mte -gcflags all=-N -l .: # mte/app                        
  ../../app/ibc.go:10:2: "github.com/cosmos/cosmos-sdk/types/module" imported and     
  not used                                                                            
  : exit status 1                                                                     
  
  Waiting for a fix before retrying...
  
  Press the 'q' key to stop serve
```

Press `q` to stop the chain.

Output:

```text
  💿 Genesis state saved in /home/user/.ignite/local-chains/mte/exported_genesis.json
  
  𝓲 Stopped
  
  💬 Survey: https://bit.ly/3WZS2uS
```

Remove the unused import:

```shell
$ sed -i '10d' app/ibc.go
```

Start the chain again:

```shell
$ ignite chain serve
```

Output:

```text
  Blockchain is running
  
  👤 alice's account address: cosmos1243r4wftmxufaxqh6lar4hgf38j8405kh2ptyd
  👤 bob's account address: cosmos1u7cgk2k0cza2s8zfmwmk7sc7mq3y8vevfdwuez
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: /home/user/.mte
  ⋆ App binary: /home/user/go/bin/mted
  
  Press the 'q' key to stop serve
```

Press `q` to stop the chain.

Output:

```text
✘ panic: collections: not found: key 'no_key' of type github.com/cosmos/gogoproto/cosmwasm.wasm.v1.Params                                                                                                    
                                                                                                                                                                                                           
goroutine 109 [running]:                                                                                                                                                                                   
github.com/CosmWasm/wasmd/x/wasm/keeper.Keeper.GetParams({{0x59866e0, 0xc001450be0}, {0x59fc3a8, 0xc000bffc80}, {0x59a02c0, 0xc0024f6640}, {0x59861a0, 0xc002701a30}, {0x59858e0, 0xc002706978}, ...}, ...)
        /home/user/go/pkg/mod/github.com/!cosm!wasm/wasmd@v0.50.0/x/wasm/keeper/keeper.go:124 +0x252                                                                                                             
github.com/CosmWasm/wasmd/x/wasm/keeper.ExportGenesis({{0x59c2348, 0x7536ea0}, {0x59db008, 0xc0029a7ac0}, {{0x0, 0x0}, {0x0, 0x0}, 0x139, {0x0, ...}, ...}, ...}, ...)                                     
        /home/user/go/pkg/mod/github.com/!cosm!wasm/wasmd@v0.50.0/x/wasm/keeper/genesis.go:89 +0x106                                                                                                             
github.com/CosmWasm/wasmd/x/wasm.AppModule.ExportGenesis({{}, {0x59fc3a8, 0xc000bffc80}, 0xc000b32d00, {0x5985960, 0xc001317700}, {0x59a02c0, 0xc0024f6a00}, {0x7f1430968760, 0xc000aba960}, ...}, ...)    
        /home/user/go/pkg/mod/github.com/!cosm!wasm/wasmd@v0.50.0/x/wasm/module.go:194 +0x7b                                                                                                                     
github.com/cosmos/cosmos-sdk/types/module.(*Manager).ExportGenesisForModules.func3({0x7f142ebca310, 0xc002718230}, 0xc0021b2380)                                                                           
        /home/user/go/pkg/mod/github.com/cosmos/cosmos-sdk@v0.50.9/types/module/module.go:584 +0x135                                                                                                             
created by github.com/cosmos/cosmos-sdk/types/module.(*Manager).ExportGenesisForModules in goroutine 1                                                                                                     
        /home/user/go/pkg/mod/github.com/cosmos/cosmos-sdk@v0.50.9/types/module/module.go:582 +0xbc9                                                                                                             
: exit status 2              
```

This time the chain stopped with an error, to get rid of this, type:

```shell
$ ignite chain serve --reset-once
```

Output:

```text
  Blockchain is running
  
  ✔ Added account alice with address cosmos1cy0x6ax4a3je5nsf266xvldcf5qdq7rsps35a0 and mnemonic:
  treat limit sibling control civil save box talent they memory forward fame scare mimic medal spread venture copper man glory cream minor cradle garment
  
  ✔ Added account bob with address cosmos17lrcp5h9q37pvms2gm7mnln6kqvugjdajrgen5 and mnemonic:
  saddle drop bonus panel pioneer front logic belt often educate distance melt craft pave rate cabbage paddle light charge solution punch grief trouble hover
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: /home/user/.mte
  ⋆ App binary: /home/user/go/bin/mted
  
  Press the 'q' key to stop serve
```

Press `q` to stop the chain.

Output:

```text
  💿 Genesis state saved in /home/user/.ignite/local-chains/mte/exported_genesis.json
  
  𝓲 Stopped
  
  💬 Survey: https://bit.ly/3WZS2uS
```

## Deploying a `counter` smart contract

In the directory [mte-counter](./mte-counter) there is an example **counter** smart contract prepared.

Firstly, we need to compile this contract to Wasm.

```shell
$ cd mte-counter
```

In `mte-counter` directory type: 

```shell
$ cargo wasm
```

The contract will be compiled to Wasm.
The compiled binary named `counter.wasm` can be found in directory `target/wasm32-unknown-unknown/release/counter.wasm`.

Start the chain:

```shell
$ ignite chain serve
```

Output:

```text
  Blockchain is running
  
  👤 alice's account address: cosmos1cy0x6ax4a3je5nsf266xvldcf5qdq7rsps35a0
  👤 bob's account address: cosmos17lrcp5h9q37pvms2gm7mnln6kqvugjdajrgen5
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: /home/confio/.mte
  ⋆ App binary: /home/confio/go/bin/mted
  
  Press the 'q' key to stop serve
```

Open a new terminal and change to `mte-counter` directory.

Store the contract's binary on a chain:

```shell
$ mted tx wasm store ./target/wasm32-unknown-unknown/release/counter.wasm --from alice --chain-id mte --gas 10000000 -y
```

Output:

```text
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: ""
timestamp: ""
tx: null
txhash: B2793CEB1F8B60B726C64BED43DF4744A5C9175201807E633EC9B5D6D7C6448E
```

Query stored wasm binaries:

```shell
$ mted q wasm list-code
```

Output:

```text
code_infos:
- code_id: "1"
  creator: cosmos1cy0x6ax4a3je5nsf266xvldcf5qdq7rsps35a0
  data_hash: E3999E6E26B67D96B60E07B52AA5D7A7B92B18C90DA3DE9762A002490721CB3F
  instantiate_permission:
    addresses: []
    permission: Everybody
pagination:
  next_key: null
  total: "0"
```

🎉 Alice has successfully stored a smart contract on the chain!

Having the `code_id` of the contract, we can check the code details:

```shell
$ mted q wasm code-info 1
```

Output:

```text
code_id: "1"
creator: cosmos1cy0x6ax4a3je5nsf266xvldcf5qdq7rsps35a0
data_hash: E3999E6E26B67D96B60E07B52AA5D7A7B92B18C90DA3DE9762A002490721CB3F
instantiate_permission:
  addresses: []
  permission: Everybody
```

Now, let's instantiate a new contract based on counter contract code with `code_id = 1`.
This contract instance will be initialized with zero.

```shell
$ mted tx wasm instantiate 1 '"zero"' --label my-counter --no-admin --from alice --chain-id mte -y
```

```shell
$ mted tx wasm instantiate 1 '{"set":21}' --label my-counter --no-admin --from alice --chain-id mte -y
```



```shell
$ mted query tx --type=hash D9E9270B186C86B3325448F8F1FBE2D328BA7678783FBE15F0952D55C28A5825 | yq 'del(.tx.body.messages[0].wasm_byte_code)'
$ mted query tx --type=hash 23FC4501EF1B473298323DC5B6E376078F791D8348D8FB30E38085B91BF086AA | yq 'del(.tx.body.messages[0].wasm_byte_code)'
```


```shell
$ mted q wasm contract-state smart cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '"value"'
```

```shell
$ mted q wasm list-contract-by-code 1
$ mted q wasm contract-state smart cosmos1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrs2zhgh2 '"value"'
```

```shell
$ mted tx wasm execute cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '"inc"' --from alice --chain-id mte -y
```
