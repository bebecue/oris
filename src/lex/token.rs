#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Token {
    pub(crate) pos: usize,
    pub(crate) kind: Kind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    /// `let`
    Let,

    /// `true`
    True,

    /// `false`
    False,

    /// `fn`
    Fn,

    /// `return`
    Return,

    /// `if`
    If,

    /// `else`
    Else,

    /// `foobar`
    ///
    /// identifier
    Ident(Box<str>),

    /// `42`
    Int(i32),

    /// `"hello, world"`
    ///
    /// UTF-8 string
    Str(Box<str>),

    /// `,`
    Comma,

    /// `:`
    Colon,

    /// `;`
    Semicolon,

    /// `(`
    LeftParen,

    /// `)`
    RightParen,

    /// `[`
    LeftBracket,

    /// `]`
    RightBracket,

    /// `{`
    LeftBrace,

    /// `}`
    RightBrace,

    /// `+`
    Plus,

    /// `-`
    Hyphen,

    /// `*`
    Asterisk,

    /// `/`
    Slash,

    /// `=`
    Assign,

    /// `!`
    Bang,

    /// `==`
    Eq,

    /// `!=`
    Ne,

    /// `<`
    Lt,

    /// `>`
    Gt,

    /// `<=`
    Le,

    /// `>=`
    Ge,
}
