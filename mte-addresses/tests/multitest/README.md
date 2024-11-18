# Legend

- CW_PREFIX = `"cosmwasm"`;
- CW_BECH32 = `"cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y"`;
- CW_BECH32M = `"cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsl5lc5x"`;
- NB_PREFIX = `"nebula"`;
- NB_BECH32 = `"nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsvsqrvp"`;
- NB_BECH32M = `"nebula1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsevs0fr"`;

# Full matrix

```text
┌───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┐
│(1)│      (2)       │          (3)          │       (4)      │    (5)    │    (6)     │(7)│
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_app_api.rs                                                                          │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ app.api()      │ App::default()        │       -        │     -     │ CW_BECH32  │ T │
│ 2 │ app.api()      │ App::default()        │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32  │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ CW_PREFIX │ CW_BECH32M │ T │
│ 5 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32  │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_addr.rs                                                                        │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ into_addr()    │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 2 │ into_addr()    │          -            │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ into_addr()    │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ into_addr()    │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ N │
│ 5 │ into_addr()    │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ into_addr()    │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ N │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32.rs                                                                      │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ into_bech32()  │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 2 │ into_bech32()  │          -            │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ into_bech32()  │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ into_bech32()  │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ N │
│ 5 │ into_bech32()  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ into_bech32()  │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ N │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32m.rs                                                                     │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ into_bech32m() │          -            │       -        │     -     │ CW_BECH32  │ N │
│ 2 │ into_bech32m() │          -            │       -        │     -     │ CW_BECH32M │ T │
│ 3 │ into_bech32m() │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ N │
│ 4 │ into_bech32m() │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ T │
│ 5 │ into_bech32m() │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ N │
│ 6 │ into_bech32m() │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api.rs                                                                         │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ MockApi        │ MockApi::default      │       -        │     -     │ CW_BECH32  │ T │
│ 2 │ MockApi        │ MockApi::default      │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ MockApi        │ MockApi::with_prefix  │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ MockApi        │ MockApi::with_prefix  │       -        │ CW_PREFIX │ CW_BECH32M │ N │
│ 5 │ MockApi        │ MockApi::with_prefix  │       -        │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ MockApi        │ MockApi::with_prefix  │       -        │ NB_PREFIX │ NB_BECH32M │ N │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32.rs                                                                  │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ MockApiBech32  │          -            │       -        │     -     │ CW_BECH32  │ N │
│ 2 │ MockApiBech32  │          -            │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ MockApiBech32  │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ MockApiBech32  │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ N │
│ 5 │ MockApiBech32  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ MockApiBech32  │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ N │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32m.rs                                                                 │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ MockApiBech32m │          -            │       -        │     -     │ CW_BECH32  │ N │
│ 2 │ MockApiBech32m │          -            │       -        │     -     │ CW_BECH32M │ N │
│ 3 │ MockApiBech32m │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ N │
│ 4 │ MockApiBech32m │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ T │
│ 5 │ MockApiBech32m │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ N │
│ 6 │ MockApiBech32m │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
└───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┘
```

# Matrix reduced to existing tests cases 

```text
┌───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┐
│(1)│      (2)       │          (3)          │       (4)      │    (5)    │    (6)     │(7)│
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_app_api.rs                                                                          │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ app.api()      │ App::default()        │       -        │     -     │ CW_BECH32  │ T │
│ 3 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32  │ CW_PREFIX │ CW_BECH32  │ T │
│ 4 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ CW_PREFIX │ CW_BECH32M │ T │
│ 5 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32  │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_addr.rs                                                                        │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ into_addr()    │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 3 │ into_addr()    │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 5 │ into_addr()    │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32.rs                                                                      │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ into_bech32()  │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 3 │ into_bech32()  │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 5 │ into_bech32()  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32m.rs                                                                     │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 2 │ into_bech32m() │          -            │       -        │     -     │ CW_BECH32M │ T │
│ 4 │ into_bech32m() │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ T │
│ 6 │ into_bech32m() │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api.rs                                                                         │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ MockApi        │ MockApi::default      │       -        │     -     │ CW_BECH32  │ T │
│ 3 │ MockApi        │ MockApi::with_prefix  │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 5 │ MockApi        │ MockApi::with_prefix  │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32.rs                                                                  │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 3 │ MockApiBech32  │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 5 │ MockApiBech32  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32m.rs                                                                 │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 4 │ MockApiBech32m │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ T │
│ 6 │ MockApiBech32m │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
└───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┘
```

# Matrix reduced to tests cases included in documentation 

```text
┌───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┐
│(1)│      (2)       │          (3)          │       (4)      │    (5)    │    (6)     │(7)│
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_app_api.rs                                                                          │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ app.api()      │ App::default()        │       -        │     -     │ CW_BECH32  │ T │
│ 4 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ CW_PREFIX │ CW_BECH32M │ T │
│ 5 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32  │ NB_PREFIX │ NB_BECH32  │ T │
│ 6 │ app.api()      │ AppBuilder.with_api() │ MockApiBech32m │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_addr.rs                                                                        │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤
│ 1 │ into_addr()    │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 5 │ into_addr()    │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32.rs                                                                      │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ into_bech32()  │          -            │       -        │     -     │ CW_BECH32  │ T │
│ 5 │ into_bech32()  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_into_bech32m.rs                                                                     │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 2 │ into_bech32m() │          -            │       -        │     -     │ CW_BECH32M │ T │
│ 6 │ into_bech32m() │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api.rs                                                                         │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 1 │ MockApi        │ MockApi::default      │       -        │     -     │ CW_BECH32  │ T │
│ 5 │ MockApi        │ MockApi::with_prefix  │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32.rs                                                                  │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 3 │ MockApiBech32  │          -            │       -        │ CW_PREFIX │ CW_BECH32  │ T │
│ 5 │ MockApiBech32  │          -            │       -        │ NB_PREFIX │ NB_BECH32  │ T │
├───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┤
│ test_mock_api_bech32m.rs                                                                 │
├───┬────────────────┬───────────────────────┬────────────────┬───────────┬────────────┬───┤ 
│ 4 │ MockApiBech32m │          -            │       -        │ CW_PREFIX │ CW_BECH32M │ T │
│ 6 │ MockApiBech32m │          -            │       -        │ NB_PREFIX │ NB_BECH32M │ T │
└───┴────────────────┴───────────────────────┴────────────────┴───────────┴────────────┴───┘
```
