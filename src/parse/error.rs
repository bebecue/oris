use crate::lex;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Error {
    Lex(lex::Error),
    Miss(Box<Expected>),
    Mismatch(Box<Mismatch>),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct Mismatch {
    pub(crate) left: lex::Token,
    pub(crate) right: Expected,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Expected {
    Expr,
    Ident,
    Token(lex::Token),
}

impl Error {
    pub(crate) fn miss(expected: Expected) -> Self {
        Self::Miss(Box::new(expected))
    }

    pub(crate) fn mismatch(left: lex::Token, right: Expected) -> Self {
        Self::Mismatch(Box::new(Mismatch { left, right }))
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
            Error::Miss(expected) => {
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
