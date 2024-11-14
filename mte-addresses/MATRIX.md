```text

 1 | app.api()      | App::default()        |       -        |     -     | CW_BECH32  | T |
 2 | app.api()      | App::default()        |       -        |     -     | CW_BECH32M | N |
 3 | app.api()      | AppBuilder.with_api() | MockApiBech32  | CW_PREFIX | CW_BECH32  | T |
 4 | app.api()      | AppBuilder.with_api() | MockApiBech32m | CW_PREFIX | CW_BECH32M | T |
 5 | app.api()      | AppBuilder.with_api() | MockApiBech32  | NB_PREFIX | NB_BECH32  | T |
 6 | app.api()      | AppBuilder.with_api() | MockApiBech32m | NB_PREFIX | NB_BECH32M | T |
 
 1 | into_addr()    |          -            |       -        |     -     | CW_BECH32  | T |
 2 | into_addr()    |          -            |       -        |     -     | CW_BECH32M | N |
 3 | into_addr()    |          -            |       -        | CW_PREFIX | CW_BECH32  | T |
 4 | into_addr()    |          -            |       -        | CW_PREFIX | CW_BECH32M | N |
 5 | into_addr()    |          -            |       -        | NB_PREFIX | NB_BECH32  | T |
 6 | into_addr()    |          -            |       -        | NB_PREFIX | NB_BECH32M | N |
 
 1 | into_bech32()  |          -            |       -        |     -     | CW_BECH32  | T |
 2 | into_bech32()  |          -            |       -        |     -     | CW_BECH32M | N |
 3 | into_bech32()  |          -            |       -        | CW_PREFIX | CW_BECH32  | T |
 4 | into_bech32()  |          -            |       -        | CW_PREFIX | CW_BECH32M | N |
 5 | into_bech32()  |          -            |       -        | NB_PREFIX | NB_BECH32  | T |
 6 | into_bech32()  |          -            |       -        | NB_PREFIX | NB_BECH32M | N |
 
 1 | into_bech32m() |          -            |       -        |     -     | CW_BECH32  | N |
 2 | into_bech32m() |          -            |       -        |     -     | CW_BECH32M | T |
 3 | into_bech32m() |          -            |       -        | CW_PREFIX | CW_BECH32  | N |
 4 | into_bech32m() |          -            |       -        | CW_PREFIX | CW_BECH32M | T |
 5 | into_bech32m() |          -            |       -        | NB_PREFIX | NB_BECH32  | N |
 6 | into_bech32m() |          -            |       -        | NB_PREFIX | NB_BECH32M | T |
 
 1 | MockApi        | MockApi::default      |       -        |     -     | CW_BECH32  | T |
 2 | MockApi        | MockApi::default      |       -        |     -     | CW_BECH32M | N |
 3 | MockApi        | MockApi::with_prefix  |       -        | CW_PREFIX | CW_BECH32  | T |
 4 | MockApi        | MockApi::with_prefix  |       -        | CW_PREFIX | CW_BECH32M | N |
 5 | MockApi        | MockApi::with_prefix  |       -        | NB_PREFIX | NB_BECH32  | T |
 6 | MockApi        | MockApi::with_prefix  |       -        | NB_PREFIX | NB_BECH32M | N |

```