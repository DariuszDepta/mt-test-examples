# Using Bech32 addresses

## Cargo.toml

There are several Cargo.toml configuration files prepared for specific MultiTest/CosmWasm version combinations.

### MultiTest latest | CosmWasm latest

```toml
[dependencies]
cosmwasm-std = "2"

[dev-dependencies]
cw-multi-test = { version = "2", features = ["staking", "stargate", "cosmwasm_2_0"] }
```

```shell
$ cargo clean
$ cargo update
$ cargo test
```

```text
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### MultiTest 2.0 | CosmWasm 2.0

```toml
[dependencies]
cosmwasm-std = "2.0"

[dev-dependencies]
cw-multi-test = { version = "2.0", features = ["staking", "stargate", "cosmwasm_2_0"] }
```

```shell
$ cargo test
```

```text
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### MultiTest 1.2.0 | CosmWasm 1.5.5

```toml
[dependencies]
cosmwasm-std = "1.5.5"

[dev-dependencies]
cw-multi-test = { version = "1.2.0", features = ["cosmwasm_1_4"] }
```

```shell
$ cargo test
```

```text
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### MultiTest 1.1.0 | CosmWasm 1.5.3

```toml
[dependencies]
cosmwasm-std = "1.5.3"

[dev-dependencies]
cw-multi-test = { version = "1.1.0", features = ["cosmwasm_1_4"] }
```

```shell
$ cargo test
```

```text
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### MultiTest 1.0.0 | CosmWasm 1.5.3

```toml
[dependencies]
cosmwasm-std = "1.5.3"

[dev-dependencies]
cw-multi-test = { version = "1.0.0", features = ["cosmwasm_1_4"] }
```

```shell
$ cargo test
```

```text
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
