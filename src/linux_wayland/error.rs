use std::os::raw::c_int;
use super::{ffi, Context};
use std::fmt::{self, Display, Formatter};
use crate::{FallibleContext, utils::NonZero};

type NonZeroInt = <c_int as NonZero>::Type;

#[derive(Debug)]
pub struct PlatformError(NonZeroInt);

impl PlatformError {
    pub(super) fn errno() -> Self {
        unsafe {
            Self(NonZeroInt::new_unchecked(*ffi::__errno_location()))
        }
    }
}

impl Display for PlatformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unsafe {
            let string = ffi::strerror(self.0.get());
            let len = ffi::strlen(string);
            let message = std::slice::from_raw_parts(string, len);
            match std::str::from_utf8(message) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => write!(f, "Error code: {}", self.0.get()),
            }
        }
    }
}

impl std::error::Error for PlatformError {}

impl FallibleContext for Context {
    type PlatformError = PlatformError;
}
