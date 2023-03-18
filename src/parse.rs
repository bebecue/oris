pub(crate) mod ast;
mod error;
mod parser;

#[cfg(test)]
mod tests;

pub(crate) type Parser<'a> = parser::Parser<'a>;

pub(crate) type Error = error::Error;
pub(crate) type Result<T> = std::result::Result<T, error::Error>;
