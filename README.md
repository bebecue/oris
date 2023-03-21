[![crates.io](https://img.shields.io/crates/v/oris.svg)](https://crates.io/crates/oris)
[![documentation](https://docs.rs/oris/badge.svg)](https://docs.rs/oris)

# `oris`

An interpreter for [Monkey](https://monkeylang.org/)

## Install

```text
cargo install oris
```

## REPL

```text
$ oris
>> 1 + 1
2
>>
```

## Script

```text
$ cat dt.oris
let answer = 2 * 3 * 7;
print(answer);

$ oris dt.oris
42
```

## Embedded

```rust
let code = b"
let is_composite = fn(n) {
    let f = fn(d) {
        if n <= d {
            false
        } else {
            let q = n / d;
            if q * d == n {
                true
            } else {
                f(d + 1)
            }
        }
    }

    f(2)
}

let sum = fn(m) {
    let f = fn(n) {
        if n == m {
            0
        } else {
            if is_composite(n) {
                0
            } else {
                n
            } + f(n + 1)
        }
    };

    f(1)
}

sum(limit)
";

let mut env = oris::Env::builder().with_int("limit", 14).build();

let result = oris::entry(&mut env, code).unwrap();

assert_eq!(result.as_int().unwrap(), 42);
```
