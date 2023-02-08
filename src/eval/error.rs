use crate::{eval::value::Value, parse::ast};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Error {
    AssertEq(Value, Value),
    Parse(crate::parse::Error),
    Undefined(ast::Ident),
    Index {
        container: Value,
        key: Value,
    },
    Unary {
        op: ast::UnaryOp,
        operand: Value,
    },
    Binary {
        left: Value,
        op: ast::BinaryOp,
        right: Value,
    },
    Call(Value),
    ArgCount {
        supplied: usize,
        expected: usize,
    },
    ArgType {
        supplied: Value,
        expected: &'static str,
    },
    ArgValue(&'static str),
}

impl Error {
    pub(super) fn unary(op: ast::UnaryOp, operand: Value) -> Self {
        Self::Unary { op, operand }
    }

    pub(super) fn binary(left: Value, op: ast::BinaryOp, right: Value) -> Self {
        Self::Binary { left, op, right }
    }

    pub(super) fn index(container: Value, key: Value) -> Self {
        Self::Index { container, key }
    }
}

impl From<crate::parse::Error> for Error {
    fn from(err: crate::parse::Error) -> Self {
        Self::Parse(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::AssertEq(left, right) => {
                f.write_str("assert_eq failed\n")?;
                write!(f, " left: {:?}\n", left)?;
                write!(f, "right: {:?}", right)?;
                Ok(())
            }
            Self::Undefined(ident) => {
                write!(f, "undefined identifier: {}", ident)
            }
            Self::Index { container, key } => {
                write!(f, "index {:?} with {:?}", container, key)
            }
            Self::Unary { op, operand } => {
                write!(f, "invalid unary operator {} for {:?}", op, operand)
            }
            Self::Binary { left, op, right } => {
                write!(
                    f,
                    "invalid binary operator {} between {:?} and {:?}",
                    op, left, right
                )
            }
            Self::Call(kind) => {
                write!(f, "{:?} is not callable", kind)
            }
            Self::ArgCount { supplied, expected } => {
                write!(f, "accept arg x {}, but got {}", expected, supplied)
            }
            Self::ArgType { supplied, expected } => {
                write!(f, "accept arg of type {}, but got {:?}", expected, supplied)
            }
            Self::ArgValue(message) => f.write_str(message),
            Self::Parse(error) => error.fmt(f),
        }
    }
}
