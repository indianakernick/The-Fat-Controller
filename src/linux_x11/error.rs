use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    OpenDisplay,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Error::*;
        match self {
            OpenDisplay => write!(f, "Error opening display"),
        }
    }
}
