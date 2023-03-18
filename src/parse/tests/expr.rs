use super::*;

fn ident(name: &str) -> Expr {
    Expr::Ident(Ident::test(name))
}

fn unary(op: UnaryOp, value: Expr) -> Expr {
    Expr::Unary(Box::new(Unary { pos: 0, op, value }))
}

fn pic_eq(left: &Node, right: &Expr) -> bool {
    match left {
        Node::Stmt(_) => false,
        Node::Expr(left) => pic_eq_expr(left, right),
    }
}

macro_rules! t {
    ($input:literal, $($right:expr),*) => {
        let nodes = parse($input).unwrap();

        let mut i = 0;

        $(
            match (&nodes[i], $right) {
                (left, right) if !pic_eq(left, &right) => {
                    panic!("mismatch at index {}: {:?} != {:?}", i, left, right);
                }
                _ => {
                    i += 1;
                }
            }
        )*;

        assert_eq!(nodes.len(), i);
    };
}

#[test]
fn int_() {
    t!("0", int(0));
    t!("01", int(1));
    t!("2", int(2));
    t!("123456", int(123456));
}

#[test]
fn bool_() {
    t!("true", bool(true));
    t!("false", bool(false));
}

#[test]
fn str_() {
    t!("\"foo\"", str("foo"));
}

#[test]
fn seq_() {
    t!(
        "[2, a, true, \"b\"]",
        seq(vec![int(2), ident("a"), bool(true), str("b")])
    );
}

#[test]
fn map_() {
    t!(
        "{1: a, true: \"b\", \"three\": 3}",
        Expr::Map(Map {
            pos: 0,
            entries: vec![
                (int(1), ident("a")),
                (bool(true), str("b")),
                (str("three"), int(3))
            ]
            .into_boxed_slice()
        })
    );
}

#[test]
fn ident_() {
    t!("a", ident("a"));
    t!("a0", ident("a0"));
    t!("a_0", ident("a_0"));
    t!("_a", ident("_a"));

    // TODO: match on specific error
    //
    // TODO: std::assert_matches::assert_matches is unstable
    assert!(matches!(
        parse("1a").unwrap_err(),
        crate::parse::Error::Lex(_)
    ));
}

#[test]
fn unary_() {
    t!("-1", unary(UnaryOp::Neg, int(1)));
    t!("-a", unary(UnaryOp::Neg, ident("a")));

    t!("!true", unary(UnaryOp::Not, bool(true)));
    t!("!false", unary(UnaryOp::Not, bool(false)));
    t!("!a", unary(UnaryOp::Not, ident("a")));
}

#[test]
fn binary_() {
    t!("1 + 2", binary(int(1), BinaryOp::Add, int(2)));
    t!("1 - 2", binary(int(1), BinaryOp::Sub, int(2)));
    t!("1 * 2", binary(int(1), BinaryOp::Mul, int(2)));
    t!("1 / 2", binary(int(1), BinaryOp::Div, int(2)));

    t!("a + b", binary(ident("a"), BinaryOp::Add, ident("b")));
    t!("a - b", binary(ident("a"), BinaryOp::Sub, ident("b")));
    t!("a * b", binary(ident("a"), BinaryOp::Mul, ident("b")));
    t!("a / b", binary(ident("a"), BinaryOp::Div, ident("b")));
}

#[test]
fn precedence() {
    t!(
        "1 + 2 * 3",
        binary(int(1), BinaryOp::Add, binary(int(2), BinaryOp::Mul, int(3)))
    );

    t!(
        "1 + 2 * 3 - 4",
        binary(
            binary(int(1), BinaryOp::Add, binary(int(2), BinaryOp::Mul, int(3))),
            BinaryOp::Sub,
            int(4)
        )
    );

    t!(
        "(1 + 2) * 3",
        binary(binary(int(1), BinaryOp::Add, int(2)), BinaryOp::Mul, int(3),)
    );
}
