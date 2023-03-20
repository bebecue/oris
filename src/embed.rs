use std::fmt;

use crate::eval;

pub type Result = std::result::Result<Value, Error>;

pub fn entry(env: &mut Env, code: &[u8]) -> Result {
    let value = eval::entry(&mut env.inner, code)?;
    Ok(Value { value })
}

pub struct Env {
    inner: eval::Env,
}

impl Env {
    pub fn new() -> Self {
        Self {
            inner: eval::Env::with_builtin(),
        }
    }
}

pub struct Value {
    value: eval::Value,
}

impl Value {
    pub fn is_unit(&self) -> bool {
        matches!(self.value, eval::Value::Unit)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

pub struct Error {
    inner: eval::Error,
}

impl Error {
    /// both line and column number are 0 based
    ///
    /// # Panics
    ///
    /// panics if `code` is not the original source code where this error is
    /// produced
    pub fn line_column(&self, code: &[u8]) -> (usize, usize) {
        pos_to_line_column(self.inner.pos(), code)
    }
}

fn pos_to_line_column(pos: usize, code: &[u8]) -> (usize, usize) {
    match code[..pos].iter().rposition(|b| *b == b'\n') {
        Some(i) => {
            let (lines, column_text) = code[..pos].split_at(i + 1);
            let line_number = lines.iter().filter(|b| **b == b'\n').count();
            (line_number, column_text.len()) // TODO: support UTF-8?
        }
        None => (0, code[..pos].len()), // TODO: support UTF-8?
    }
}

impl From<eval::Error> for Error {
    fn from(error: eval::Error) -> Self {
        Self { inner: error }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
impl std::error::Error for Error {}
