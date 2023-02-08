use super::*;

macro_rules! t {
    ($input:literal, $output:expr) => {
        let lexer = crate::lex::Lexer::new($input.as_bytes().into());
        let mut parser = crate::parse::Parser::new(lexer);
        assert_eq!(parser.next(), Some(Ok(Node::Expr($output))));
        assert_eq!(parser.next(), None);
    };
}

macro_rules! t_err {
    ($input:literal, $output:expr) => {
        let lexer = crate::lex::Lexer::new($input.as_bytes().into());
        let mut parser = crate::parse::Parser::new(lexer);
        assert_eq!(parser.next(), Some(Err($output)));
    };
}

fn ident(name: &str) -> Expr {
    Expr::Ident(Ident::new(name.into()))
}

fn unary(op: UnaryOp, expr: Expr) -> Expr {
    Expr::Unary(op, Box::new(expr))
}

#[test]
fn int_() {
    t!("0", Expr::Int(0));
    t!("01", Expr::Int(1));
    t!("2", Expr::Int(2));
    t!("123456", Expr::Int(123456));
}

#[test]
fn bool_() {
    t!("true", Expr::Bool(true));
    t!("false", Expr::Bool(false));
}

#[test]
fn str_() {
    t!("\"foo\"", Expr::Str("foo".into()));
}

#[test]
fn seq_() {
    t!(
        "[2, a, true, \"b\"]",
        Expr::Seq(
            vec![
                Expr::Int(2),
                ident("a"),
                Expr::Bool(true),
                Expr::Str("b".into())
            ]
            .into_boxed_slice()
        )
    );
}

#[test]
fn map_() {
    t!(
        "{1: a, true: \"b\", \"three\": 3}",
        Expr::Map(
            vec![
                (Expr::Int(1), ident("a")),
                (Expr::Bool(true), Expr::Str("b".into())),
                (Expr::Str("three".into()), Expr::Int(3))
            ]
            .into_boxed_slice()
        )
    );
}

#[test]
fn ident_() {
    t!("a", ident("a"));
    t!("a0", ident("a0"));
    t!("a_0", ident("a_0"));
    t!("_a", ident("_a"));

    t_err!(
        "1a",
        crate::parse::Error::Lex(crate::lex::Error::BadDigit(b'a'))
    );
}

#[test]
fn unary_() {
    t!("-1", unary(UnaryOp::Neg, Expr::Int(1)));
    t!("-a", unary(UnaryOp::Neg, ident("a")));

    t!("!true", unary(UnaryOp::Not, Expr::Bool(true)));
    t!("!false", unary(UnaryOp::Not, Expr::Bool(false)));
    t!("!a", unary(UnaryOp::Not, ident("a")));
}

#[test]
fn binary_() {
    t!("1 + 2", binary(Expr::Int(1), BinaryOp::Add, Expr::Int(2)));
    t!("1 - 2", binary(Expr::Int(1), BinaryOp::Sub, Expr::Int(2)));
    t!("1 * 2", binary(Expr::Int(1), BinaryOp::Mul, Expr::Int(2)));
    t!("1 / 2", binary(Expr::Int(1), BinaryOp::Div, Expr::Int(2)));

    t!("a + b", binary(ident("a"), BinaryOp::Add, ident("b")));
    t!("a - b", binary(ident("a"), BinaryOp::Sub, ident("b")));
    t!("a * b", binary(ident("a"), BinaryOp::Mul, ident("b")));
    t!("a / b", binary(ident("a"), BinaryOp::Div, ident("b")));
}

#[test]
fn precedence() {
    t!(
        "1 + 2 * 3",
        binary(
            Expr::Int(1),
            BinaryOp::Add,
            binary(Expr::Int(2), BinaryOp::Mul, Expr::Int(3))
        )
    );

    t!(
        "1 + 2 * 3 - 4",
        binary(
            binary(
                Expr::Int(1),
                BinaryOp::Add,
                binary(Expr::Int(2), BinaryOp::Mul, Expr::Int(3))
            ),
            BinaryOp::Sub,
            Expr::Int(4)
        )
    );

    t!(
        "(1 + 2) * 3",
        binary(
            binary(Expr::Int(1), BinaryOp::Add, Expr::Int(2)),
            BinaryOp::Mul,
            Expr::Int(3),
        )
    );
}
