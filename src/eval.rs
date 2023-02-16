mod binary;
mod env;
mod error;
mod value;

#[cfg(test)]
mod tests;

use std::rc::Rc;

use crate::parse::ast;

pub(crate) type Env = env::Env;
pub(crate) type Value = value::Value;

pub(crate) type Error = error::Error;
type Result<T> = std::result::Result<T, self::error::Error>;

pub(crate) fn entry(env: &mut Env, code: Vec<u8>) -> Result<Value> {
    let lexer = crate::lex::Lexer::new(code.into_boxed_slice());
    let parser = crate::parse::Parser::new(lexer);

    let mut output = Value::Unit;

    for node in parser {
        let node = node?;

        match eval_node(env, &node)? {
            Eval::Continue(x) => output = x,
            Eval::Return(x) => return Ok(x),
        }
    }
    Ok(output)
}

enum Eval<T = Value> {
    Continue(T),
    Return(Value),
}

fn eval_node(env: &mut Env, node: &ast::Node) -> Result<Eval> {
    match node {
        ast::Node::Expr(expr) => eval_expr(env, expr),
        ast::Node::Stmt(stmt) => eval_stmt(env, stmt).map(|r| match r {
            Eval::Continue(()) => Eval::Continue(Value::Unit),
            Eval::Return(v) => Eval::Return(v),
        }),
    }
}

fn eval_stmt(env: &mut Env, stmt: &ast::Stmt) -> Result<Eval<()>> {
    match stmt {
        ast::Stmt::Let(ident, value) => {
            let value = propagate!(eval_expr(env, value));

            let value = match value {
                Value::Closure(mut closure) => {
                    if let Some(i) = closure.undefined.iter().position(|ident_| ident_ == ident) {
                        let c = Rc::get_mut(&mut closure).unwrap();
                        c.recursive = Some(c.undefined.remove(i));
                    }
                    Value::Closure(closure)
                }
                other => other,
            };

            env.set(ident.clone(), value);
            Ok(()).map(Eval::Continue)
        }
        ast::Stmt::Return(stmt) => match stmt {
            None => Ok(Eval::Continue(())),
            Some(ref expr) => eval_expr(env, expr).map(|r| match r {
                Eval::Continue(v) | Eval::Return(v) => Eval::Return(v),
            }),
        },
    }
}

fn eval_expr(env: &mut Env, expr: &ast::Expr) -> Result<Eval> {
    match expr {
        ast::Expr::Int(x) => Ok(Value::Int(*x)).map(Eval::Continue),
        ast::Expr::Bool(x) => Ok(Value::Bool(*x)).map(Eval::Continue),
        ast::Expr::Str(x) => Ok(Value::Str(Rc::clone(x))).map(Eval::Continue),
        ast::Expr::Ident(ident) => env
            .get(ident)
            .cloned()
            .ok_or_else(|| Error::Undefined(ident.clone()))
            .map(Eval::Continue),
        ast::Expr::Seq(seq) => {
            let mut elements = Vec::with_capacity(seq.len());
            for expr in seq.iter() {
                let elem = propagate!(eval_expr(env, expr));
                elements.push(elem);
            }
            Ok(Value::Seq(elements.into())).map(Eval::Continue)
        }
        ast::Expr::Unary(op, expr) => {
            let value = propagate!(eval_expr(env, expr));

            match op {
                ast::UnaryOp::Neg => match value {
                    Value::Int(x) => Ok(Value::Int(-x)),
                    other => Err(Error::unary(*op, other)),
                },
                ast::UnaryOp::Not => match value {
                    Value::Bool(x) => Ok(Value::Bool(!x)),
                    other => Err(Error::unary(*op, other)),
                },
            }
            .map(Eval::Continue)
        }
        ast::Expr::Binary(left, op, right) => {
            let left = propagate!(eval_expr(env, left));
            let right = propagate!(eval_expr(env, right));
            binary::eval(left, *op, right).map(Eval::Continue)
        }
        ast::Expr::Closure(closure) => Ok(value::Closure::new(Rc::clone(closure), env))
            .map(Rc::new)
            .map(Value::Closure)
            .map(Eval::Continue),
        ast::Expr::Map(entries) => {
            let mut map = std::collections::HashMap::with_capacity(entries.len());

            for (k, v) in entries.iter() {
                let k = propagate!(eval_expr(env, k));
                match value::to_key(&k) {
                    Some(k) => {
                        let v = propagate!(eval_expr(env, v));
                        map.insert(k, v);
                    }
                    None => {
                        return Err(Error::ArgType {
                            supplied: k,
                            expected: "int | bool | str as map key",
                        });
                    }
                }
            }

            Ok(map).map(Rc::from).map(Value::Map).map(Eval::Continue)
        }
        ast::Expr::Call(f, args) => eval_call(env, f, args),
        ast::Expr::Index(target, key) => eval_index(env, target, key),
        ast::Expr::If(expr) => eval_if(env, expr),
    }
}

fn eval_call(env: &mut Env, target: &ast::Expr, args: &[ast::Expr]) -> Result<Eval> {
    fn eval_args(env: &mut Env, args: &[ast::Expr]) -> Result<Eval<Vec<Value>>> {
        let mut values = Vec::with_capacity(args.len());

        for arg in args {
            let value = propagate!(eval_expr(env, arg));
            values.push(value);
        }

        Ok(values).map(Eval::Continue)
    }

    let target = propagate!(eval_expr(env, target));
    let args = propagate!(eval_args(env, args));

    match target {
        Value::Closure(closure) => {
            if args.len() != closure.f.parameters.len() {
                Err(Error::ArgCount {
                    supplied: args.len(),
                    expected: closure.f.parameters.len(),
                })
            } else {
                env.enclosed(|env| {
                    if let Some(ref name) = closure.recursive {
                        env.set(name.clone(), Value::Closure(Rc::clone(&closure)));
                    }

                    for (ident, value) in closure.captured.iter() {
                        env.set(ident.clone(), value.clone());
                    }

                    for (ident, arg) in std::iter::zip(closure.f.parameters.iter(), args) {
                        env.set(ident.clone(), arg);
                    }

                    match eval_block(env, &closure.f.body)? {
                        Eval::Continue(v) | Eval::Return(v) => Ok(v),
                    }
                })
            }
        }
        Value::Builtin(f) => f(args),
        _ => Err(Error::Call(target)),
    }
    .map(Eval::Continue)
}

fn eval_block(env: &mut Env, block: &ast::Block) -> Result<Eval> {
    let mut result = Value::Unit;
    for node in block.nodes.iter() {
        result = propagate!(eval_node(env, node));
    }
    Ok(Eval::Continue(result))
}

fn eval_if(env: &mut Env, expr: &ast::If) -> Result<Eval> {
    let condition = propagate!(eval_expr(env, &expr.condition));

    if let Value::Bool(true) = condition {
        eval_block(env, &expr.consequence)
    } else {
        match expr.alternative {
            Some(ref expr) => eval_block(env, expr),
            None => Ok(Value::Unit).map(Eval::Continue),
        }
    }
}

fn eval_index(env: &mut Env, target: &ast::Expr, key: &ast::Expr) -> Result<Eval> {
    let target = propagate!(eval_expr(env, target));
    let key = propagate!(eval_expr(env, key));

    match target {
        Value::Seq(seq) => {
            let i = match key {
                Value::Int(i) => i,
                other => return Err(Error::index(Value::Seq(seq), other)),
            };

            let iusize = match usize::try_from(i) {
                Ok(iusize) => iusize,
                Err(_) => return Err(Error::index(Value::Seq(seq), Value::Int(i))),
            };

            match seq.get(iusize) {
                Some(value) => Ok(value.clone()),
                _ => Err(Error::index(Value::Seq(seq), Value::Int(i))),
            }
        }
        Value::Map(map) => value::to_key(&key)
            .and_then(|key| map.get(&key).cloned())
            .ok_or_else(|| Error::index(Value::Map(map), key)),
        other => Err(Error::index(other, key)),
    }
    .map(Eval::Continue)
}
