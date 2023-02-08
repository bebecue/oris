# `oris`

An interpreter for the [Monkey](https://monkeylang.org/) programming language

## Examples

```
# line comment starts with `#`

# integer

let positive = 42;     # use `let <identifier> = <expression>` to declare variable
let negative = -3;
let zero = 7 + -7;

# boolean

let yes = !false;
let no  = true == false;

# string

let description = "Oris is an interpreter for the monkey programming language";

# sequence

let seq = [true, 2, 3, 5, 6 + 1, "hello, world"];

# map
# key can be integer, boolean or string
let pairs = {
        1: true,
    false: 4,
    "key": "value"
};

# function

let double = fn(x) {
    x * 2    # last expression's value becomes function's return value
};

assert_eq(double(3), 6);
# `assert_eq` is a builtin function that stops interpreting
# if the two arguments are not equal to each other

# closure (function can also capture values from environment)
let map = fn(seq, mapper) {
    let iter = fn(from, to) {
        if len(from) == 0 {     # () around condition is optional
            return to;
        }

        iter(                   # call iter recursively
            tail(from),         # built-in `tail` returns elements after the first one
            append(             # built-in `append` add one element to tail and return the new sequence
                to,
                mapper(         # call the captured `mapper` function
                    head(from)  # built-in `head` returns the first element
                )
            )
        )
    }

    iter(seq, [])               # `to` starts with an empty sequence
};

assert_eq(map([1, 2, 3], double), [2, 4, 6]);

# calculate fibnacci number
# 1, 1, 2, 3, 5, 8, 13, 21
let fib = fn(x) {
    if x < 2 {    
        return 1;
    }

    fib(x - 1) + fib(x - 2)
};

print(fib(7));                  # use built-in function `print` to write the result `21` to stdout
```
