use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    XOpenDisplay,
    XQueryPointer,
    XWarpPointer,
    XTestQueryExtension,
    XTestFakeKeyEvent,
    XTestFakeButtonEvent,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Error::*;
        match self {
            XOpenDisplay => write!(f, "Error opening display"),
            XQueryPointer => write!(f, "Cursor is not in main screen"),
            XWarpPointer => write!(f, "Error moving cursor"),
            XTestQueryExtension => write!(f, "XTest extension is unavailable"),
            XTestFakeKeyEvent => write!(f, "Error pressing key"),
            XTestFakeButtonEvent => write!(f, "Error pressing mouse button"),
        }
    }
}
