use std::rc::Rc;

use crate::{
    eval::{self, Value},
    parse::ast,
};

pub(super) fn eval(left: Value, op: ast::BinaryOp, right: Value) -> eval::Result<Value> {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => Ok(int_(left, op, right)),
        (Value::Bool(left), Value::Bool(right)) => bool_(left, op, right),
        (Value::Str(left), Value::Str(right)) => str_(left, op, right),
        (Value::Seq(left), Value::Seq(right)) => seq_(left, op, right),
        (Value::Map(left), Value::Map(right)) => map_(left, op, right),
        (left, right) => Err(eval::Error::binary(left, op, right)),
    }
}

fn int_(left: i32, op: ast::BinaryOp, right: i32) -> Value {
    match op {
        ast::BinaryOp::Add => Value::Int(left + right),
        ast::BinaryOp::Sub => Value::Int(left - right),
        ast::BinaryOp::Mul => Value::Int(left * right),
        ast::BinaryOp::Div => Value::Int(left / right),
        ast::BinaryOp::Lt => Value::Bool(left < right),
        ast::BinaryOp::Le => Value::Bool(left <= right),
        ast::BinaryOp::Gt => Value::Bool(left > right),
        ast::BinaryOp::Ge => Value::Bool(left >= right),
        ast::BinaryOp::Eq => Value::Bool(left == right),
        ast::BinaryOp::Ne => Value::Bool(left != right),
    }
}

fn bool_(left: bool, op: ast::BinaryOp, right: bool) -> eval::Result<Value> {
    match op {
        ast::BinaryOp::Eq => Ok(Value::Bool(left == right)),
        ast::BinaryOp::Ne => Ok(Value::Bool(left != right)),
        _ => Err(eval::Error::binary(
            Value::Bool(left),
            op,
            Value::Bool(right),
        )),
    }
}

fn str_(left: Rc<str>, op: ast::BinaryOp, right: Rc<str>) -> eval::Result<Value> {
    match op {
        ast::BinaryOp::Add => {
            let mut new_str = String::with_capacity(left.len() + right.len());
            new_str += &left;
            new_str += &right;
            Ok(Value::Str(new_str.into()))
        }
        ast::BinaryOp::Eq => Ok(Value::Bool(left == right)),
        ast::BinaryOp::Ne => Ok(Value::Bool(left != right)),
        _ => Err(eval::Error::binary(Value::Str(left), op, Value::Str(right))),
    }
}

fn seq_(left: Rc<[Value]>, op: ast::BinaryOp, right: Rc<[Value]>) -> eval::Result<Value> {
    match op {
        ast::BinaryOp::Add => {
            let mut new_seq = Vec::with_capacity(left.len() + right.len());
            new_seq.extend(left.iter().cloned());
            new_seq.extend(right.iter().cloned());
            Ok(Value::Seq(new_seq.into()))
        }
        ast::BinaryOp::Eq => Ok(Value::Bool(left == right)),
        ast::BinaryOp::Ne => Ok(Value::Bool(left != right)),
        _ => Err(eval::Error::binary(Value::Seq(left), op, Value::Seq(right))),
    }
}

fn map_(
    left: Rc<std::collections::HashMap<super::value::Key, Value>>,
    op: ast::BinaryOp,
    right: Rc<std::collections::HashMap<super::value::Key, Value>>,
) -> eval::Result<Value> {
    match op {
        ast::BinaryOp::Eq => Ok(Value::Bool(left == right)),
        ast::BinaryOp::Ne => Ok(Value::Bool(left != right)),
        _ => Err(eval::Error::binary(Value::Map(left), op, Value::Map(right))),
    }
}
