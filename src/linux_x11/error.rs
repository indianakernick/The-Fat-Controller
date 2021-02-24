use super::Context;
use crate::FallibleContext;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum PlatformError {
    XOpenDisplay,
    XQueryPointer,
    XWarpPointer,
    XTestQueryExtension,
    XTestFakeKeyEvent,
    XTestFakeButtonEvent,
    KeySymToUnicode,
    XGetKeyboardMapping,
    NoUnusedKeyCode,
    XkbGetMap,
    XGetModifierMapping,
}

impl Display for PlatformError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use PlatformError::*;
        match self {
            XOpenDisplay => write!(f, "Error opening display"),
            XQueryPointer => write!(f, "Cursor is not in main screen"),
            XWarpPointer => write!(f, "Error moving cursor"),
            XTestQueryExtension => write!(f, "XTest extension is unavailable"),
            XTestFakeKeyEvent => write!(f, "Error pressing key"),
            XTestFakeButtonEvent => write!(f, "Error pressing mouse button"),
            KeySymToUnicode => write!(f, "Failed to map keysym to unicode scalar"),
            XGetKeyboardMapping => write!(f, "Failed to get keyboard mapping"),
            NoUnusedKeyCode => write!(f, "Couldn't find a keycode with no associated keysyms"),
            XkbGetMap => write!(f, "Failed to get keyboard information"),
            XGetModifierMapping => write!(f, "Failed to get modifier key mapping"),
        }
    }
}

impl std::error::Error for PlatformError {}

impl FallibleContext for Context {
    type PlatformError = PlatformError;
}
