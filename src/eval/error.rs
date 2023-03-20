use crate::{eval::value::Value, parse::ast};

#[derive(Debug)]
pub(crate) enum Error {
    AssertEq {
        pos: usize,
        left: Value,
        right: Value,
    },
    Parse(crate::parse::Error),
    Undefined(ast::Ident),
    Index {
        pos: usize,
        base: Value,
        subscript: Value,
    },
    Unary {
        pos: usize,
        op: ast::UnaryOp,
        operand: Value,
    },
    Binary {
        pos: usize,
        left: Value,
        op: ast::BinaryOp,
        right: Value,
    },
    Call {
        pos: usize,
        target: Value,
        args: Box<[Value]>,
    },
    ArgCount {
        pos: usize,
        supplied: usize,
        expected: usize,
    },
    ArgType {
        pos: usize,
        supplied: Value,
        expected: &'static str,
    },
    ArgValue {
        pos: usize,
        message: &'static str,
    },
}

impl Error {
    pub(crate) fn pos(&self) -> usize {
        match self {
            Self::AssertEq { pos, .. } => *pos,
            Self::Parse(error) => error.pos(),
            Self::Undefined(ident) => ident.pos(),
            Self::Index { pos, .. } => *pos,
            Self::Unary { pos, .. } => *pos,
            Self::Binary { pos, .. } => *pos,
            Self::Call { pos, .. } => *pos,
            Self::ArgCount { pos, .. } => *pos,
            Self::ArgType { pos, .. } => *pos,
            Self::ArgValue { pos, .. } => *pos,
        }
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
            Self::AssertEq {
                pos: _,
                left,
                right,
            } => {
                f.write_str("assert_eq failed\n")?;
                write!(f, " left: {:?}\n", left)?;
                write!(f, "right: {:?}", right)?;
                Ok(())
            }
            Self::Undefined(ident) => {
                write!(f, "undefined identifier: {}", ident)
            }
            Self::Index {
                pos: _,
                base,
                subscript,
            } => {
                write!(f, "index {:?} with {:?}", base, subscript)
            }
            Self::Unary {
                pos: _,
                op,
                operand,
            } => {
                write!(f, "invalid unary operator {} for {:?}", op, operand)
            }
            Self::Binary {
                pos: _,
                left,
                op,
                right,
            } => {
                write!(
                    f,
                    "invalid binary operator {} between {:?} and {:?}",
                    op, left, right
                )
            }
            Self::Call { target, .. } => {
                write!(f, "{:?} is not callable", target)
            }
            Self::ArgCount {
                pos: _,
                supplied,
                expected,
            } => {
                write!(f, "accept arg x {}, but got {}", expected, supplied)
            }
            Self::ArgType {
                pos: _,
                supplied,
                expected,
            } => {
                write!(f, "accept arg of type {}, but got {:?}", expected, supplied)
            }
            Self::ArgValue { pos: _, message } => f.write_str(message),
            Self::Parse(error) => error.fmt(f),
        }
    }
}
