//! `oris` is an interpreter for the [Monkey](https://monkeylang.org/) programming language
//!
//! # Examples
//!
//! ```
//! let mut env = oris::Env::new();
//!
//! let code = r#"
//!     print("hello, world");
//! "#;
//!
//! oris::entry(&mut env, code.into()).unwrap();
//! ```

#[macro_use]
mod macros;

mod embed;
mod eval;
mod lex;
mod parse;

pub use embed::{entry, Env, Error, Result};
