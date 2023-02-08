use super::*;

macro_rules! t {
    ($input:literal, $tokens:expr) => {
        let mut lexer = Lexer::new($input.as_bytes().into());
        for token in $tokens {
            assert_eq!(lexer.next(), Some(Ok(token)));
        }
        assert_eq!(lexer.next(), None);
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
            Token::Let,
            Token::True,
            Token::False,
            Token::Fn,
            Token::Return,
            Token::If,
            Token::Else,
            Token::Ident("foo".into()),
            Token::Int(0),
            Token::Int(1),
            Token::Int(42),
            Token::Str("bar".into()),
            Token::Comma,
            Token::Colon,
            Token::Semicolon,
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBracket,
            Token::RightBracket,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Plus,
            Token::Hyphen,
            Token::Asterisk,
            Token::Slash,
            Token::Assign,
            Token::Bang,
            Token::Eq,
            Token::Ne,
            Token::Lt,
            Token::Gt,
            Token::Le,
            Token::Ge,
        ]
    );
}

#[test]
fn stmts() {
    t!(
        "let answer = 42;",
        vec![
            Token::Let,
            Token::Ident("answer".into()),
            Token::Assign,
            Token::Int(42),
            Token::Semicolon,
        ]
    );

    t!(
        "let is = true;",
        vec![
            Token::Let,
            Token::Ident("is".into()),
            Token::Assign,
            Token::True,
            Token::Semicolon
        ]
    );

    t!(
        "let not = false;",
        vec![
            Token::Let,
            Token::Ident("not".into()),
            Token::Assign,
            Token::False,
            Token::Semicolon
        ]
    );

    t!(
        "let arith = 1 + 4 * 3 / 2;",
        vec![
            Token::Let,
            Token::Ident("arith".into()),
            Token::Assign,
            Token::Int(1),
            Token::Plus,
            Token::Int(4),
            Token::Asterisk,
            Token::Int(3),
            Token::Slash,
            Token::Int(2),
            Token::Semicolon,
        ]
    );

    t!(
        "let min = fn(x, y) { if x < y { x } else { y } };",
        vec![
            Token::Let,
            Token::Ident("min".into()),
            Token::Assign,
            Token::Fn,
            Token::LeftParen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::RightParen,
            Token::LeftBrace,
            Token::If,
            Token::Ident("x".into()),
            Token::Lt,
            Token::Ident("y".into()),
            Token::LeftBrace,
            Token::Ident("x".into()),
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Ident("y".into()),
            Token::RightBrace,
            Token::RightBrace,
            Token::Semicolon,
        ]
    );

    t!(
        r#"let seq = [true, "==", !false];"#,
        vec![
            Token::Let,
            Token::Ident("seq".into()),
            Token::Assign,
            Token::LeftBracket,
            Token::True,
            Token::Comma,
            Token::Str("==".into()),
            Token::Comma,
            Token::Bang,
            Token::False,
            Token::RightBracket,
            Token::Semicolon,
        ]
    );

    t!(
        r#"let map = {"not true": false, 2: "two", false: 4, "seq": []};"#,
        vec![
            Token::Let,
            Token::Ident("map".into()),
            Token::Assign,
            Token::LeftBrace,
            Token::Str("not true".into()),
            Token::Colon,
            Token::False,
            Token::Comma,
            Token::Int(2),
            Token::Colon,
            Token::Str("two".into()),
            Token::Comma,
            Token::False,
            Token::Colon,
            Token::Int(4),
            Token::Comma,
            Token::Str("seq".into()),
            Token::Colon,
            Token::LeftBracket,
            Token::RightBracket,
            Token::RightBrace,
            Token::Semicolon,
        ]
    );
}

macro_rules! t_err {
    ($input:literal, $error:expr) => {
        let mut lexer = Lexer::new($input.as_bytes().into());
        assert_eq!(lexer.next(), Some(Err($error)));
    };
}

#[test]
fn errors() {
    t_err!("\"abc", Error::Quote);
    t_err!("123456789123456789", Error::Overflow);
    t_err!("12g", Error::BadDigit(b'g'));
    t_err!("0x", Error::BadDigit(b'x'));
    t_err!("$", Error::Unexpected(b'$'));
}
