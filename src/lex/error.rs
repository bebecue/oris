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

    BadDigit(u8),

    Unexpected(u8),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            Kind::Quote => f.write_str("missing right quote for string literal"),
            Kind::Overflow => f.write_str("integer literal is too large"),
            Kind::BadDigit(d) => write!(f, "bad digit `{}` in integer literal", char::from(d)),
            Kind::Unexpected(b) => write!(f, "unexpected byte ({}){:#04x}", char::from(b), b),
        }
    }
}
