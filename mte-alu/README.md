MultiTest | Examples

# Arithmetic logic unit

The purpose of this project is following:

- show how to configure the logic of the smart contract during instantiation,
- show how to use submessages,
- diagnose/confirm the handling of the data returned from executing the contract,
- diagnose/confirm the handling of message responses in replies,
- diagnose/confirm the handling of events,
- diagnose/confirm the handling of attributes.

## Design

### (1) Initializing the logic of the smart contract

Operations:

| Operation | Data  |
|:---------:|-------|
|    Nop    | null  | 
|    Add    | x + y |
|    Sub    | x - y |
|    Mul    | x * y |
|    Div    | x / y |

## Execution scenarios

### Contract without submessages

Calling `execute` endpoint should return the result of the calculation as `data`.
Data is stored in transaction as `data` field. Check, how the events and attributes are stored.
Confirm, that the behaviour of MultiTest is exactly the same. Check if the AppResponse contains
exactly the same set of data, events and attributes as returned in the transaction on chain.

```text
 █   Contract returns some/none of data/events/attributes in `Response`
 └── No submessages returned from `execute` entry-point
```

### Contract with single submessage

```text
 █<────────┐ 
 └──▒──>█──┘ 
```

### Contract with multiple submessages

```text
 █<────────┐
 ├──▒──>█──┤
 ├──▒──>█──┤
 ├──▒──>█──┤
 ├──▒──>█──┤
 ├──▒──>█──┤
 ├──▒──>█──┤
 └──▒──>█──┘
```

### Contract with nested submessages

```text
 █<─────┐
 └──▒──>█<─────┐
        └──▒──>█<─────┐
               └──▒──>█<─────┐ 
                      └──▒──>█<─────┐
                             └──▒──>█
```

### Contract with multiple nested submessages

```text
 █<─────┐<────────────────────────────┐<─┐<─┐<─┐<─┐
 ├──▒──>█<─────┐                      │  │  │  │  │
 │      └──▒──>█<─────┐               │  │  │  │  │
 │             └──▒──>█<─────┐        │  │  │  │  │
 │                    └──▒──>█<─────┐ │  │  │  │  │
 │                           └──▒──>█ │  │  │  │  │
 │      ┌─────────────────────────────┘  │  │  │  │
 ├──▒──>█                                │  │  │  │
 │      ┌────────────────────────────────┘  │  │  │
 ├──▒──>█<─────┐                            │  │  │
 │      └──▒──>█<─────┐                     │  │  │
 │             └──▒──>█                     │  │  │
 │      ┌───────────────────────────────────┘  │  │
 ├──▒──>█<─────┐                               │  │
 │      └──▒──>█<─────┐                        │  │ 
 │             └──▒──>█<─────┐                 │  │
 │                    └──▒──>█                 │  │
 │      ┌──────────────────────────────────────┘  │
 ├──▒──>█<─────┐                                  │
 │      └──▒──>█                                  │
 │      ┌─────────────────────────────────────────┘
 └──▒──>█
```
