use crate::parse::{ast::*, tests::*};

fn ident(name: &str) -> Ident {
    Ident::test(name)
}

fn pic_eq(left: &Node, right: &Stmt) -> bool {
    match left {
        Node::Expr(_) => false,
        Node::Stmt(left) => pic_eq_stmt(left, right),
    }
}

macro_rules! t {
    ($input:literal, $output:expr) => {
        let nodes = parse($input).unwrap();
        match $output {
            output if !pic_eq(&nodes[0], &output) => {
                panic!("stmt mismatch: {:?} != {:?}", nodes[0], output);
            }
            _ => {}
        }
        assert_eq!(nodes.len(), 1);
    };
}

#[test]
fn let_() {
    t!(
        "let a = 42;",
        Stmt::Let(Let {
            pos: 0,
            ident: ident("a"),
            value: int(42)
        })
    );
    t!(
        "let b = true;",
        Stmt::Let(Let {
            pos: 0,
            ident: ident("b"),
            value: bool(true)
        })
    );
    t!(
        "let c = false;",
        Stmt::Let(Let {
            pos: 0,
            ident: ident("c"),
            value: bool(false)
        })
    );
    t!(
        "let math = 1 + 4 * 3 - 6 / 2;",
        Stmt::Let(Let {
            pos: 0,
            ident: ident("math"),
            value: binary(
                binary(int(1), BinaryOp::Add, binary(int(4), BinaryOp::Mul, int(3))),
                BinaryOp::Sub,
                binary(int(6), BinaryOp::Div, int(2))
            )
        })
    );
    t!(
        "let always_five = fn() { 5 };",
        Stmt::Let(Let {
            pos: 0,
            ident: ident("always_five"),
            value: Expr::Closure(
                Closure {
                    pos: 0,
                    parameters: Box::new([]),
                    body: block(vec![Node::Expr(int(5))]),
                }
                .into()
            )
        })
    );
    t!(
        "let add = fn(x, y) { x + y };",
        Stmt::Let(Let {
            pos: 0,
            ident: ident("add"),
            value: Expr::Closure(
                Closure {
                    pos: 0,
                    parameters: [ident("x"), ident("y")].into(),
                    body: block(vec![binary(
                        Expr::Ident(ident("x")),
                        BinaryOp::Add,
                        Expr::Ident(ident("y")),
                    )]),
                }
                .into()
            )
        })
    );
}

#[test]
fn return_() {
    t!(
        "return;",
        Stmt::Return(Return {
            pos: 0,
            value: None
        })
    );
    t!(
        "return 1;",
        Stmt::Return(Return {
            pos: 0,
            value: Some(int(1))
        })
    );
}
