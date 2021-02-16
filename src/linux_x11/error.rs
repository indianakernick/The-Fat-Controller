use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    OpenDisplay,
    XTestQuery,
    QueryPointer,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Error::*;
        match self {
            OpenDisplay => write!(f, "Error opening display"),
            XTestQuery => write!(f, "XTest extension is unavailable"),
            QueryPointer => write!(f, "Cursor is not in main screen"),
        }
    }
}
