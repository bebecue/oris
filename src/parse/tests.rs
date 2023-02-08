mod expr;
mod stmt;

use super::ast::*;

fn binary(left: Expr, op: BinaryOp, right: Expr) -> Expr {
    Expr::Binary(Box::new(left), op, Box::new(right))
}

fn block<N>(nodes: Vec<N>) -> Block
where
    Node: From<N>,
{
    Block {
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
