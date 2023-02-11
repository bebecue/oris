use crate::eval;

pub type Result = std::result::Result<(), Error>;

pub fn entry(env: &mut Env, code: Vec<u8>) -> Result {
    eval::entry(&mut env.inner, code)?;
    Ok(())
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

pub struct Error {
    inner: eval::Error,
}

impl From<eval::Error> for Error {
    fn from(error: eval::Error) -> Self {
        Self { inner: error }
    }
}

use std::fmt;
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
