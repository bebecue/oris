mod error;
mod lexer;
mod token;

#[cfg(test)]
mod tests;

pub(crate) type Lexer = lexer::Lexer;

pub(crate) type Token = token::Token;

pub(crate) type Error = error::Error;
pub(crate) type Result<T> = std::result::Result<T, self::error::Error>;
