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
let mut env = oris::Env::new();

let code = b"
let fib = fn(n) {
    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
};

fib(10)
";

let value = oris::entry(&mut env, code).unwrap();
assert_eq!(value.as_int().unwrap(), 89);
```
