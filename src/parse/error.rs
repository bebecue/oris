use crate::lex;

#[derive(Debug)]
pub(crate) enum Error {
    Lex(lex::Error),
    Incomplete(Incomplete),
    Mismatch(Mismatch),
}

#[derive(Debug)]
pub(crate) struct Incomplete {
    pub(crate) pos: usize,
    pub(crate) expected: Expected,
}

#[derive(Debug)]
pub(crate) struct Mismatch {
    pub(crate) left: lex::token::Token,
    pub(crate) right: Expected,
}

#[derive(Debug)]
pub(crate) enum Expected {
    Expr,
    Token(lex::token::Kind),
}

impl Error {
    pub(crate) fn pos(&self) -> usize {
        match self {
            Self::Lex(error) => error.pos,
            Self::Incomplete(incomplete) => incomplete.pos,
            Self::Mismatch(mismatch) => mismatch.left.pos,
        }
    }
}

impl From<lex::Error> for Error {
    fn from(error: lex::Error) -> Self {
        Self::Lex(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Lex(error) => error.fmt(f),
            Error::Incomplete(incomplete) => {
                write!(f, "miss {}", incomplete.expected)
            }
            Error::Mismatch(error) => {
                write!(f, "expect {}, found {:?}", error.right, error.left)
            }
        }
    }
}

impl std::fmt::Display for Expected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expr => f.write_str("expression"),
            Self::Token(token) => write!(f, "{:?}", token),
        }
    }
}
