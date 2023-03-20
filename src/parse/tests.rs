mod expr;
mod stmt;

use super::ast::*;

fn bool(value: bool) -> Expr {
    Expr::Bool(Bool { pos: 0, value })
}

fn int(value: i32) -> Expr {
    Expr::Int(Int { pos: 0, value })
}

fn str(value: &str) -> Expr {
    Expr::Str(Str::from_src(0, value))
}

fn seq(elements: Vec<Expr>) -> Expr {
    Expr::Seq(Seq {
        pos: 0,
        elements: elements.into_boxed_slice(),
    })
}

fn binary(left: Expr, op: BinaryOp, right: Expr) -> Expr {
    Expr::Binary(Box::new(Binary {
        pos: 0,
        left,
        op,
        right,
    }))
}

fn block<N>(nodes: Vec<N>) -> Block
where
    Node: From<N>,
{
    Block {
        pos: 0,
        nodes: nodes
            .into_iter()
            .map(Node::from)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    }
}

impl From<Stmt> for Node {
    fn from(stmt: Stmt) -> Node {
        Node::Stmt(stmt)
    }
}

impl From<Expr> for Node {
    fn from(expr: Expr) -> Node {
        Node::Expr(expr)
    }
}

fn parse(input: &'static str) -> crate::parse::Result<Vec<Node>> {
    let lexer = crate::lex::Lexer::new(input.as_bytes().into());
    let parser = crate::parse::Parser::new(lexer);
    parser.collect()
}

fn pic_eq_node(left: &Node, right: &Node) -> bool {
    match (left, right) {
        (Node::Expr(left), Node::Expr(right)) => pic_eq_expr(left, right),
        (Node::Stmt(left), Node::Stmt(right)) => pic_eq_stmt(left, right),
        _ => false,
    }
}

fn pic_eq_stmt(left: &Stmt, right: &Stmt) -> bool {
    match (left, right) {
        (Stmt::Let(left), Stmt::Let(right)) => {
            left.ident.sym() == right.ident.sym() && pic_eq_expr(&left.value, &right.value)
        }
        (Stmt::Return(left), Stmt::Return(right)) => {
            pic_eq_opt(left.value.as_ref(), right.value.as_ref(), pic_eq_expr)
        }
        _ => false,
    }
}

fn pic_eq_expr(left: &Expr, right: &Expr) -> bool {
    match (left, right) {
        (Expr::Int(left), Expr::Int(right)) => left.value == right.value,
        (Expr::Bool(left), Expr::Bool(right)) => left.value == right.value,
        (Expr::Str(left), Expr::Str(right)) => left.value() == right.value(),
        (Expr::Seq(left), Expr::Seq(right)) => {
            pic_eq_slice(&left.elements, &right.elements, pic_eq_expr)
        }
        (Expr::Map(left), Expr::Map(right)) => pic_eq_slice(
            &left.entries,
            &right.entries,
            |(left_key, left_value), (right_key, right_value)| {
                pic_eq_expr(left_key, right_key) && pic_eq_expr(left_value, right_value)
            },
        ),
        (Expr::Ident(left), Expr::Ident(right)) => left.sym() == right.sym(),
        (Expr::Index(left), Expr::Index(right)) => {
            pic_eq_expr(&left.base, &right.base) && pic_eq_expr(&left.subscript, &right.subscript)
        }
        (Expr::Unary(left), Expr::Unary(right)) => {
            left.op == right.op && pic_eq_expr(&left.value, &right.value)
        }
        (Expr::Binary(left), Expr::Binary(right)) => {
            pic_eq_expr(&left.left, &right.left)
                && left.op == right.op
                && pic_eq_expr(&left.right, &right.right)
        }
        (Expr::Closure(left), Expr::Closure(right)) => {
            pic_eq_slice(&*left.parameters, &*right.parameters, |left, right| {
                left.sym() == right.sym()
            }) && pic_eq_block(&left.body, &right.body)
        }
        (Expr::Call(left), Expr::Call(right)) => {
            pic_eq_expr(&left.target, &right.target)
                && pic_eq_slice(&left.args, &right.args, pic_eq_expr)
        }
        (Expr::If(left), Expr::If(right)) => {
            pic_eq_expr(&left.condition, &right.condition)
                && pic_eq_block(&left.consequence, &right.consequence)
                && pic_eq_opt(
                    left.alternative.as_ref(),
                    right.alternative.as_ref(),
                    pic_eq_block,
                )
        }
        _ => false,
    }
}

fn pic_eq_block(left: &Block, right: &Block) -> bool {
    pic_eq_slice(&left.nodes, &right.nodes, pic_eq_node)
}

// TODO: Iterator::pic_eq_by() is unstable
fn pic_eq_slice<T, F>(left: &[T], right: &[T], compare: F) -> bool
where
    F: Fn(&T, &T) -> bool,
{
    left.len() == right.len()
        && std::iter::zip(left, right).all(|(left, right)| compare(left, right))
}

fn pic_eq_opt<T, F>(left: Option<&T>, right: Option<&T>, compare: F) -> bool
where
    F: FnOnce(&T, &T) -> bool,
{
    match (left, right) {
        (Some(left), Some(right)) => compare(left, right),
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (None, None) => true,
    }
}
