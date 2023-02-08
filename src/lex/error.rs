#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Error {
    /// miss the right `"` in string
    Quote,

    /// integer literal is too large
    Overflow,

    BadDigit(u8),

    Unexpected(u8),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quote => f.write_str("missing right quote for string literal"),
            Self::Overflow => f.write_str("integer literal is too large"),
            Self::BadDigit(d) => write!(f, "bad digit `{}` in integer literal", char::from(*d)),
            Self::Unexpected(b) => write!(f, "unexpected byte ({}){:#04x}", char::from(*b), b),
        }
    }
}
