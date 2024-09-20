# Scaffolding a  blockchain

[Go]: https://go.dev/doc/install
[IgniteCLI]: https://docs.ignite.com/welcome/install

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
  
  ⋆ Data directory: /home/confio/.mte
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
  
  ⋆ Data directory: /home/confio/.mte
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
