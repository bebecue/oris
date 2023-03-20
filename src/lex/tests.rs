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
            Kind::Ident,
            Kind::Int,
            Kind::Int,
            Kind::Int,
            Kind::Str,
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
            Kind::Ident,
            Kind::Assign,
            Kind::Int,
            Kind::Semicolon,
        ]
    );

    t!(
        "let is = true;",
        vec![
            Kind::Let,
            Kind::Ident,
            Kind::Assign,
            Kind::True,
            Kind::Semicolon
        ]
    );

    t!(
        "let not = false;",
        vec![
            Kind::Let,
            Kind::Ident,
            Kind::Assign,
            Kind::False,
            Kind::Semicolon
        ]
    );

    t!(
        "let arith = 1 + 4 * 3 / 2;",
        vec![
            Kind::Let,
            Kind::Ident,
            Kind::Assign,
            Kind::Int,
            Kind::Plus,
            Kind::Int,
            Kind::Asterisk,
            Kind::Int,
            Kind::Slash,
            Kind::Int,
            Kind::Semicolon,
        ]
    );

    t!(
        "let min = fn(x, y) { if x < y { x } else { y } };",
        vec![
            Kind::Let,
            Kind::Ident,
            Kind::Assign,
            Kind::Fn,
            Kind::LeftParen,
            Kind::Ident,
            Kind::Comma,
            Kind::Ident,
            Kind::RightParen,
            Kind::LeftBrace,
            Kind::If,
            Kind::Ident,
            Kind::Lt,
            Kind::Ident,
            Kind::LeftBrace,
            Kind::Ident,
            Kind::RightBrace,
            Kind::Else,
            Kind::LeftBrace,
            Kind::Ident,
            Kind::RightBrace,
            Kind::RightBrace,
            Kind::Semicolon,
        ]
    );

    t!(
        r#"let seq = [true, "==", !false];"#,
        vec![
            Kind::Let,
            Kind::Ident,
            Kind::Assign,
            Kind::LeftBracket,
            Kind::True,
            Kind::Comma,
            Kind::Str,
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
            Kind::Ident,
            Kind::Assign,
            Kind::LeftBrace,
            Kind::Str,
            Kind::Colon,
            Kind::False,
            Kind::Comma,
            Kind::Int,
            Kind::Colon,
            Kind::Str,
            Kind::Comma,
            Kind::False,
            Kind::Colon,
            Kind::Int,
            Kind::Comma,
            Kind::Str,
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
    t_err!("12g", error::Kind::BadDigit);
    t_err!("0x", error::Kind::BadDigit);
    t_err!("$", error::Kind::Unexpected);
}
