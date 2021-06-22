use std::fmt::{self, Debug, Display, Formatter};

/// Error enum returned by the [`traits`](crate::traits).
#[derive(Debug)]
pub enum GenericError<P: std::error::Error> {
    /// Platform-specific error.
    ///
    /// This is likely to be an integer error code. The [`Display`]
    /// implementation will provide a description of the error while the
    /// [`Debug`] implementation will only provide a number.
    Platform(P),
    /// Unsupported ASCII character.
    ///
    /// This is returned by
    /// [`AsciiKeyboardContext`](crate::AsciiKeyboardContext) when an
    /// unsupported or invalid character is given.
    UnsupportedAscii(u8),
    /// Unsupported Unicode character.
    ///
    /// This is returned by
    /// [`UnicodeKeyboardContext`](crate::UnicodeKeyboardContext) when an
    /// unsupported or invalid character is given.
    UnsupportedUnicode(char),
    /// Unknown error.
    ///
    /// This is returned when an underlying function doesn't return an error
    /// code but still indicates failure in some way. For example, if a function
    /// returns a null pointer when a non-null pointer was expected, this
    /// `Unknown` error will be returned.
    Unknown,
}

impl<P: std::error::Error> Display for GenericError<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use GenericError::*;
        match self {
            Platform(p) => write!(f, "Platform-specific error: {}", p),
            UnsupportedAscii(ch) => write!(f, "Unsupported ASCII character ({:#04X})", ch),
            UnsupportedUnicode(ch) => write!(f, "Unsupported Unicode character ('{}')", ch),
            Unknown => write!(f, "Unknown error"),
        }
    }
}

impl<P: std::error::Error> std::error::Error for GenericError<P> {}
