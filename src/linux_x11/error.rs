use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    XOpenDisplay,
    XTestQueryExtension,
    XQueryPointer,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Error::*;
        match self {
            XOpenDisplay => write!(f, "Error opening display"),
            XTestQueryExtension => write!(f, "XTest extension is unavailable"),
            XQueryPointer => write!(f, "Cursor is not in main screen"),
        }
    }
}
