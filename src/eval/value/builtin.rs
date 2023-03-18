use std::rc::Rc;

use crate::eval::{self, Value};

pub(crate) type Builtin = fn(usize, Vec<Value>) -> eval::Result<Value>;

pub(crate) fn all_() -> [(&'static str, Builtin); 7] {
    [
        ("len", len),
        ("head", head),
        ("tail", tail),
        ("append", append),
        ("print", print),
        ("assert_eq", assert_eq),
        ("type", type_),
    ]
}

// fn(str) -> int
// fn(seq) -> int
fn len(pos: usize, args: Vec<Value>) -> eval::Result<Value> {
    args!(args @ pos = value);

    match value {
        Value::Str(s) => Ok(Value::Int(s.len().try_into().expect("len as i32"))),
        Value::Seq(seq) => Ok(Value::Int(seq.len().try_into().expect("len as i32"))),
        Value::Map(map) => Ok(Value::Int(map.len().try_into().expect("len as i32"))),
        _ => Err(eval::Error::ArgType {
            pos,
            supplied: value,
            expected: "seq | str | map",
        }),
    }
}

// fn([T]) -> T
fn head(pos: usize, args: Vec<Value>) -> eval::Result<Value> {
    args!(args @ pos = value);

    match value {
        Value::Seq(seq) => match seq.first() {
            Some(x) => Ok(x.clone()),
            None => Err(eval::Error::ArgValue {
                pos,
                message: "call head() with an empty seq",
            }),
        },
        _ => Err(eval::Error::ArgType {
            pos,
            supplied: value,
            expected: "seq",
        }),
    }
}

// fn([T]) -> [T]
fn tail(pos: usize, args: Vec<Value>) -> eval::Result<Value> {
    args!(args @ pos = value);

    match value {
        Value::Seq(seq) => match seq.split_first() {
            Some((_, tail)) => Ok(tail.to_vec()).map(Rc::from).map(Value::Seq),
            None => Err(eval::Error::ArgValue {
                pos,
                message: "call tail() with an empty seq",
            }),
        },

        _ => Err(eval::Error::ArgType {
            pos,
            supplied: value,
            expected: "Seq",
        }),
    }
}

// fn([T], T...) -> [T]
fn append(pos: usize, args: Vec<Value>) -> eval::Result<Value> {
    if args.len() < 2 {
        return Err(eval::Error::ArgCount {
            pos,
            supplied: args.len(),
            expected: 2, // TODO: 2 or more
        });
    }

    let (first, tail) = args.split_first().unwrap();

    match first {
        Value::Seq(seq) => {
            let mut new_seq = Vec::with_capacity(seq.len() + tail.len());

            new_seq.extend(seq.iter().cloned());
            new_seq.extend(tail.iter().cloned());

            Ok(Value::Seq(Rc::from(new_seq)))
        }
        other => Err(eval::Error::ArgType {
            pos,
            supplied: other.clone(),
            expected: "append(seq, T...)",
        }),
    }
}

// fn(T...)
fn print(_pos: usize, args: Vec<Value>) -> eval::Result<Value> {
    if args.is_empty() {
        println!()
    }

    for arg in args {
        println!("{:?}", arg);
    }

    Ok(Value::Unit)
}

// fn(T, T)
fn assert_eq(pos: usize, args: Vec<Value>) -> eval::Result<Value> {
    args!(args @ pos = left, right);

    if left == right {
        Ok(Value::Unit)
    } else {
        Err(eval::Error::AssertEq { pos, left, right })
    }
}

// fn(T) -> str
fn type_(pos: usize, args: Vec<Value>) -> eval::Result<Value> {
    args!(args @ pos = arg);

    let name = match arg {
        Value::Unit => "unit",
        Value::Int(_) => "int",
        Value::Bool(_) => "bool",
        Value::Str(_) => "str",
        Value::Seq(_) => "seq",
        Value::Map(_) => "map",
        Value::Builtin(_) => "builtin",
        Value::Closure(_) => "closure",
    };

    Ok(Value::Str(name.into()))
}
