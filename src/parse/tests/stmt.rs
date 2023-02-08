use crate::parse::{ast::*, tests::*};

macro_rules! t {
    ($input:literal, $output:expr) => {
        let lexer = crate::lex::Lexer::new($input.as_bytes().into());
        let mut parser = crate::parse::Parser::new(lexer);
        assert_eq!(parser.next(), Some(Ok(Node::Stmt($output))));
        assert_eq!(parser.next(), None);
    };
}

fn ident(name: &str) -> Ident {
    Ident::new(name.into())
}

#[test]
fn let_() {
    t!("let a = 42;", Stmt::Let(ident("a"), Expr::Int(42)));
    t!("let b = true;", Stmt::Let(ident("b"), Expr::Bool(true)));
    t!("let c = false;", Stmt::Let(ident("c"), Expr::Bool(false)));
    t!(
        "let math = 1 + 4 * 3 - 6 / 2;",
        Stmt::Let(
            ident("math"),
            binary(
                binary(
                    Expr::Int(1),
                    BinaryOp::Add,
                    binary(Expr::Int(4), BinaryOp::Mul, Expr::Int(3))
                ),
                BinaryOp::Sub,
                binary(Expr::Int(6), BinaryOp::Div, Expr::Int(2))
            )
        )
    );
    t!(
        "let always_five = fn() { 5 };",
        Stmt::Let(
            ident("always_five"),
            Expr::Closure(
                Closure {
                    parameters: Box::new([]),
                    body: block(vec![Node::Expr(Expr::Int(5))]),
                }
                .into()
            )
        )
    );
    t!(
        "let add = fn(x, y) { x + y };",
        Stmt::Let(
            ident("add"),
            Expr::Closure(
                Closure {
                    parameters: [ident("x"), ident("y")].into(),
                    body: block(vec![binary(
                        Expr::Ident(ident("x")),
                        BinaryOp::Add,
                        Expr::Ident(ident("y")),
                    )]),
                }
                .into()
            )
        )
    );
}

#[test]
fn return_() {
    t!("return;", Stmt::Return(None));
}
