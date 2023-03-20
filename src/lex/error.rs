#[derive(Debug)]
pub(crate) struct Error {
    pub(crate) pos: usize,
    pub(crate) kind: Kind,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Kind {
    /// miss the right `"` in string
    Quote,

    /// integer literal is too large
    Overflow,

    BadDigit,

    Unexpected,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            Kind::Quote => f.write_str("missing right quote for string literal"),
            Kind::Overflow => f.write_str("integer literal is too large"),
            Kind::BadDigit => write!(f, "bad digit in integer literal"),
            Kind::Unexpected => write!(f, "unexpected byte"),
        }
    }
}
