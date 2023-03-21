use std::fmt;

use crate::eval;

pub type Result = std::result::Result<Value, Error>;

pub fn entry(env: &mut Env, code: &[u8]) -> Result {
    let value = eval::entry(&mut env.env, code)?;
    Ok(Value { value })
}

pub struct Env {
    env: eval::Env,
}

impl Env {
    pub fn new() -> Self {
        EnvBuilder::new().with_builtin().build()
    }

    pub fn builder() -> EnvBuilder {
        EnvBuilder::new()
    }
}

pub struct EnvBuilder {
    global: eval::env::Storage,
}

impl EnvBuilder {
    fn new() -> Self {
        Self {
            global: eval::env::Storage::default(),
        }
    }

    pub fn with_bool(self, name: &str, value: bool) -> Self {
        self.with_value(name, eval::Value::Bool(value))
    }

    pub fn with_int(self, name: &str, value: i32) -> Self {
        self.with_value(name, eval::Value::Int(value))
    }

    pub fn with_str(self, name: &str, value: &str) -> Self {
        self.with_value(name, eval::Value::Str(value.into()))
    }

    fn with_value(mut self, name: &str, value: eval::Value) -> Self {
        let pos = None;
        self.global.insert(std::rc::Rc::from(name), (pos, value));
        self
    }

    pub fn with_builtin(mut self) -> Self {
        for (k, v) in eval::value::builtin::all_() {
            self.global
                .insert(std::rc::Rc::from(k), (None, eval::Value::Builtin(v)));
        }

        self
    }

    pub fn build(self) -> Env {
        Env {
            env: eval::Env::new(self.global),
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

    pub fn as_bool(&self) -> Option<bool> {
        match self.value {
            eval::Value::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self.value {
            eval::Value::Int(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self.value {
            eval::Value::Str(ref value) => Some(value),
            _ => None,
        }
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
