pub(crate) mod token;

mod error;
mod lexer;

#[cfg(test)]
mod tests;

pub(crate) type Lexer<'a> = lexer::Lexer<'a>;

pub(crate) type Error = error::Error;
pub(crate) type Result<T> = std::result::Result<T, self::error::Error>;
