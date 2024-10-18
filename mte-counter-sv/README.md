MultiTest | Examples

# Counter

Counter smart contract written using Sylvia framework.

## Compiling

```shell
cargo +stable build
```

## Testing using `MultiTest`

```shell
cargo +stable test
```

```shell
cargo +stable nextest run
```

## Building WASM binary

```shell
cargo build --release --target wasm32-unknown-unknown --lib
```

## Building schema

```shell
cargo run --bin schema
```

## Testing on chain

