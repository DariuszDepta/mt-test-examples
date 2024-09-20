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

Scaffold a new chain:

```shell
$ ignite scaffold chain mte
````

_Output:_

```text
⭐️ Successfully created a new blockchain 'sevdays'.
👉 Get started with the following commands:

 % cd sevdays
 % ignite chain serve

Documentation: https://docs.ignite.com
```
