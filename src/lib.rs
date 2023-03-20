#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

mod embed;
mod eval;
mod lex;
mod parse;

pub use embed::{entry, Env, Error, Result, Value};
