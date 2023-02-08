pub(super) mod builtin;
mod closure;

use std::{collections::HashMap, fmt, rc::Rc};

pub(crate) type Builtin = builtin::Builtin;
pub(crate) type Closure = closure::Closure;

#[derive(Clone)]
pub(crate) enum Value {
    Unit,
    Int(i32),
    Bool(bool),
    Str(Rc<str>),
    Seq(Rc<[Value]>),
    Map(Rc<HashMap<Key, Value>>),
    Builtin(Builtin),
    Closure(Rc<Closure>),
}

#[derive(Hash, PartialEq, Eq)]
pub(crate) enum Key {
    Int(i32),
    Bool(bool),
    Str(Rc<str>),
}

pub(super) fn to_key(value: &Value) -> Option<Key> {
    match value {
        Value::Int(x) => Some(Key::Int(*x)),
        Value::Bool(x) => Some(Key::Bool(*x)),
        Value::Str(s) => Some(Key::Str(Rc::clone(s))),
        _ => None,
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unit => f.write_str("<unit>"),
            Value::Int(v) => v.fmt(f),
            Value::Bool(v) => v.fmt(f),
            Value::Str(v) => v.fmt(f),
            Value::Seq(seq) => f.debug_list().entries(seq.iter()).finish(),
            Value::Map(map) => f.debug_map().entries(map.iter()).finish(),
            Value::Closure(_) => f.write_str("<closure>"),
            Value::Builtin(_) => f.write_str("<builtin>"),
        }
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Int(v) => v.fmt(f),
            Key::Bool(v) => v.fmt(f),
            Key::Str(v) => v.fmt(f),
        }
    }
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit, Self::Unit) => true,
            (Self::Int(left), Self::Int(right)) => left == right,
            (Self::Bool(left), Self::Bool(right)) => left == right,
            (Self::Str(left), Self::Str(right)) => left == right,
            (Self::Seq(left), Self::Seq(right)) => left == right,
            (Self::Map(left), Self::Map(right)) => left == right,
            (Self::Builtin(left), Self::Builtin(right)) => left == right,
            (Self::Closure(left), Self::Closure(right)) => Rc::ptr_eq(left, right),
            _ => false,
        }
    }
}

impl std::cmp::Eq for Value {}
