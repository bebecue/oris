use super::*;

macro_rules! t {
    ($code:literal, $result:literal) => {
        let mut env = Env::with_builtin();

        match entry(&mut env, $code.as_bytes()) {
            Ok(Value::Int(result)) => {
                assert_eq!(result, $result);
            }
            Ok(value) => {
                panic!("expect int {}, found {:?}", $result, value);
            }
            Err(err) => {
                panic!("eval failed: {:?}", err);
            }
        }
    };

    ($code:literal, [$($elem:literal),*]) => {
        let mut env = Env::with_builtin();

        match entry(&mut env, $code.as_bytes()) {
            Ok(Value::Seq(result)) => {
                let result = result.iter().map(|v| match v {
                    Value::Int(v) => Ok(*v),
                    _ => Err(v),
                }).collect::<std::result::Result<Vec<_>,_>>();

                match result {
                    Ok(seq) => assert_eq!(seq, vec![$($elem),*]),
                    Err(value) => {
                        panic!("expect seq of int, found one is not int: {:?}", value);
                    }
                }
            }
            Ok(value) => {
                panic!("expect seq of int, found {:?}", value);
            }
            Err(err) => {
                panic!("eval failed: {:?}", err);
            }
        }
    };

    (error: $code:literal) => {
        let mut env = Env::with_builtin();

        match entry(&mut env, $code.as_bytes()) {
            result @ Ok(_) | result @ Err(Error::Parse(_)) => panic!("{:?}", result),
            _ => {}
        }
    };

    (unit: $code:literal) => {
        let mut env = Env::with_builtin();

        match entry(&mut env, $code.as_bytes()) {
            Ok(Value::Unit) => {},
            other => panic!("{:?}", other),
        }
    };

    (str: $code:literal, $result:literal) => {
        let mut env = Env::with_builtin();

        match entry(&mut env, $code.as_bytes()) {
            Ok(Value::Str(s)) => assert_eq!(&*s, $result),
            Ok(v) => panic!("expect str, found {:?}", v),
            Err(err) => panic!("eval failed: {:?}", err),
        }
    };
}

#[test]
fn arithmetic() {
    t!("1 + 2", 3);

    t!("1 - 2", -1);

    t!("2 * 3", 6);
    t!("2 * 0", 0);

    t!("2 / 3", 0);
    t!("3 / 3", 1);

    t!("1 + 2 * 3", 7);
    t!("1 + (2 * 3)", 7);
    t!("(1 + 2) * 3", 9);
    t!("(2 + 3) / 2", 2);
}

#[test]
fn if_() {
    t!("if 1 < 2 { 3 } else { 4 }", 3);
    t!("if 1 > 2 { 3 } else { 4 }", 4);

    t!("if 1 != 2 { 3 } else { 4 }", 3);
    t!("if 1 == 2 { 3 } else { 4 }", 4);

    t!("if 1 <= 2 { 3 } else { 4 }", 3);
    t!("if 1 >= 2 { 3 } else { 4 }", 4);
}

#[test]
fn closure() {
    t!("let f = fn(x) { x + 1 }; f(1)", 2);
    t!("let f = fn(x) { if x < 0 { 0 } else { x } }; f(1)", 1);
    t!("let f = fn(x) { if x < 0 { 0 } else { x } }; f(-1)", 0);
    t!(
        "
let adder = fn(x) {
    fn(y) { x + y }
};

adder(1)(2)",
        3
    );
    t!(
        "
let filter = fn(seq, predicate) {
    let iter = fn(from, to) {
        if len(from) == 0 {
            to
        } else {
            iter(tail(from), if predicate(head(from)) {
                append(to, head(from))
            } else {
                to
            })
        }
    };

    iter(seq, [])
};

filter([1, 2, 3, 4], fn(x) { x / 2 * 2 == x })
",
        [2, 4]
    );
}

#[test]
fn closure_early_return() {
    t!("let f = fn(x) { if x < 0 { return 0; } x }; f(-1)", 0);
    t!("let f = fn(x) { if x < 0 { return 0; } x }; f(1)", 1);
}

#[test]
fn seq() {
    t!("[1, 2, 3]", [1, 2, 3]);
    t!("[1] + [2, 3]", [1, 2, 3]);
    t!("[1, 2, 3][2]", 3);
}

#[test]
fn map() {
    t!(r#"{2: "two", "three": 3, false: 4}["three"]"#, 3);
    t!(r#"{2: "two", "three": 3, false: 4}[false]"#, 4);
}

#[test]
fn builtin_len() {
    t!("len([])", 0);
    t!("len([1])", 1);
    t!("len([1, 2])", 2);

    t!("len(\"\")", 0);
    t!("len(\"a\")", 1);
    t!("len(\"ab\")", 2);
    t!("len(\"a b\")", 3);

    t!("len({})", 0);
    t!("len({1: 2})", 1);
    t!("len({3: true, false: 4})", 2);

    t!(error: "len(1)");
    t!(error: "len(true)");
    t!(error: "len(len)");
    t!(error: "len(a)");
    t!(error: "len(print())");
    t!(error: "len(fn(){})");
}

#[test]
fn builtin_head() {
    t!(error: "head([])");
    t!("head([1])", 1);
    t!("head([2, 3])", 2);

    t!(error: "head({})");
    t!(error: "head(1)");
    t!(error: "head(false)");
    t!(error: "head(\"\")");
}

#[test]
fn builtin_tail() {
    t!(error: "tail([])");
    t!("tail([1])", []);
    t!("tail([1, 2])", [2]);
    t!("tail([1, 2, 3])", [2, 3]);

    t!(error: "tail({})");
    t!(error: "tail(1)");
    t!(error: "tail(false)");
    t!(error: "tail(\"\")");
}

#[test]
fn builtin_append() {
    t!(error: "append([])");
    t!("append([], 1)", [1]);
    t!("append([1], 2)", [1, 2]);
    t!("append([1], 2, 3)", [1, 2, 3]);

    t!(error: "append({})");
    t!(error: "append(1)");
    t!(error: "append(false)");
    t!(error: "append(\"\")");
}

#[test]
fn builtin_assert_eq() {
    t!(unit: "assert_eq(1, 1)");
    t!(error: "assert_eq(1, 2)");
}

#[test]
fn builtin_type() {
    t!(str: "type(print())", "unit");
    t!(str: "type(1)", "int");
    t!(str: "type(true)", "bool");
    t!(str: "type(false)", "bool");
    t!(str: "type(\"\")", "str");
    t!(str: "type([])", "seq");
    t!(str: "type([1])", "seq");
    t!(str: "type({})", "map");
    t!(str: "type({true: 1})", "map");
    t!(str: "type(type)", "builtin");
    t!(str: "type(fn(){})", "closure");
}
