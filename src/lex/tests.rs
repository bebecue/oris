use super::{error, token::Kind, Lexer};

macro_rules! t {
    ($input:literal, $tokens:expr) => {
        let mut lexer = Lexer::new($input.as_bytes().into());
        for token in $tokens {
            match lexer.next() {
                Some(Ok(tk)) => assert_eq!(tk.kind, token),
                Some(Err(err)) => panic!("expect token {:?}, found error: {:?}", token, err),
                None => panic!("expect token {:?}, found EOF", token),
            }
        }
        assert!(lexer.next().is_none());
    };
}

#[test]
fn tokens() {
    t!(
        r#"
let
true
false
fn
return
if
else
foo
0
01
42
"bar"
,
:
;
()
[]
{}
+ - * /
= !
== !=
< >
<= >=
"#,
        vec![
            Kind::Let,
            Kind::True,
            Kind::False,
            Kind::Fn,
            Kind::Return,
            Kind::If,
            Kind::Else,
            Kind::Ident("foo".into()),
            Kind::Int(0),
            Kind::Int(1),
            Kind::Int(42),
            Kind::Str("bar".into()),
            Kind::Comma,
            Kind::Colon,
            Kind::Semicolon,
            Kind::LeftParen,
            Kind::RightParen,
            Kind::LeftBracket,
            Kind::RightBracket,
            Kind::LeftBrace,
            Kind::RightBrace,
            Kind::Plus,
            Kind::Hyphen,
            Kind::Asterisk,
            Kind::Slash,
            Kind::Assign,
            Kind::Bang,
            Kind::Eq,
            Kind::Ne,
            Kind::Lt,
            Kind::Gt,
            Kind::Le,
            Kind::Ge,
        ]
    );
}

#[test]
fn stmts() {
    t!(
        "let answer = 42;",
        vec![
            Kind::Let,
            Kind::Ident("answer".into()),
            Kind::Assign,
            Kind::Int(42),
            Kind::Semicolon,
        ]
    );

    t!(
        "let is = true;",
        vec![
            Kind::Let,
            Kind::Ident("is".into()),
            Kind::Assign,
            Kind::True,
            Kind::Semicolon
        ]
    );

    t!(
        "let not = false;",
        vec![
            Kind::Let,
            Kind::Ident("not".into()),
            Kind::Assign,
            Kind::False,
            Kind::Semicolon
        ]
    );

    t!(
        "let arith = 1 + 4 * 3 / 2;",
        vec![
            Kind::Let,
            Kind::Ident("arith".into()),
            Kind::Assign,
            Kind::Int(1),
            Kind::Plus,
            Kind::Int(4),
            Kind::Asterisk,
            Kind::Int(3),
            Kind::Slash,
            Kind::Int(2),
            Kind::Semicolon,
        ]
    );

    t!(
        "let min = fn(x, y) { if x < y { x } else { y } };",
        vec![
            Kind::Let,
            Kind::Ident("min".into()),
            Kind::Assign,
            Kind::Fn,
            Kind::LeftParen,
            Kind::Ident("x".into()),
            Kind::Comma,
            Kind::Ident("y".into()),
            Kind::RightParen,
            Kind::LeftBrace,
            Kind::If,
            Kind::Ident("x".into()),
            Kind::Lt,
            Kind::Ident("y".into()),
            Kind::LeftBrace,
            Kind::Ident("x".into()),
            Kind::RightBrace,
            Kind::Else,
            Kind::LeftBrace,
            Kind::Ident("y".into()),
            Kind::RightBrace,
            Kind::RightBrace,
            Kind::Semicolon,
        ]
    );

    t!(
        r#"let seq = [true, "==", !false];"#,
        vec![
            Kind::Let,
            Kind::Ident("seq".into()),
            Kind::Assign,
            Kind::LeftBracket,
            Kind::True,
            Kind::Comma,
            Kind::Str("==".into()),
            Kind::Comma,
            Kind::Bang,
            Kind::False,
            Kind::RightBracket,
            Kind::Semicolon,
        ]
    );

    t!(
        r#"let map = {"not true": false, 2: "two", false: 4, "seq": []};"#,
        vec![
            Kind::Let,
            Kind::Ident("map".into()),
            Kind::Assign,
            Kind::LeftBrace,
            Kind::Str("not true".into()),
            Kind::Colon,
            Kind::False,
            Kind::Comma,
            Kind::Int(2),
            Kind::Colon,
            Kind::Str("two".into()),
            Kind::Comma,
            Kind::False,
            Kind::Colon,
            Kind::Int(4),
            Kind::Comma,
            Kind::Str("seq".into()),
            Kind::Colon,
            Kind::LeftBracket,
            Kind::RightBracket,
            Kind::RightBrace,
            Kind::Semicolon,
        ]
    );
}

macro_rules! t_err {
    ($input:literal, $error:expr) => {
        let mut lexer = Lexer::new($input.as_bytes().into());
        assert_eq!(lexer.next().unwrap().unwrap_err().kind, $error);
    };
}

#[test]
fn errors() {
    t_err!("\"abc", error::Kind::Quote);
    t_err!("123456789123456789", error::Kind::Overflow);
    t_err!("12g", error::Kind::BadDigit(b'g'));
    t_err!("0x", error::Kind::BadDigit(b'x'));
    t_err!("$", error::Kind::Unexpected(b'$'));
}
