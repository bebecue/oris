use crate::lex;

#[derive(Debug)]
pub(crate) enum Error {
    Lex(lex::Error),
    Incomplete(Box<Expected>),
    Mismatch(Box<Mismatch>),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct Mismatch {
    pub(crate) left: lex::token::Token,
    pub(crate) right: Expected,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Expected {
    Expr,
    Ident,
    Token(lex::token::Kind),
}

impl Error {
    // FIXME: remove `code` argument?
    pub(crate) fn pos(&self, code: &[u8]) -> usize {
        match self {
            Self::Lex(error) => error.pos,
            Self::Incomplete(_) => code.len(),
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
            Error::Incomplete(expected) => {
                write!(f, "miss {}", expected)
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
            Self::Ident => f.write_str("identifier"),
            Self::Token(token) => write!(f, "{:?}", token),
        }
    }
}
