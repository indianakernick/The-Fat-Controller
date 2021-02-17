use super::os;
use std::os::raw::c_int;
use std::fmt::{self, Display, Formatter};

type NonZeroInt = <c_int as crate::utils::NonZero>::Type;

/// Error type used throughout the library (Linux-Wayland).
///
/// The exact type depends on the platform being used. All that can be assumed
/// is that this type implements `std::error::Error`.
#[derive(Debug)]
pub struct Error(NonZeroInt);

impl Error {
    pub(super) fn errno() -> Self {
        unsafe {
            Self(NonZeroInt::new_unchecked(*os::__errno_location()))
        }
    }

    pub(super) fn unknown() -> Self {
        unsafe {
            // strerror will say "Unknown error 65535"
            Self(NonZeroInt::new_unchecked(0xFFFF))
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unsafe {
            let string = os::strerror(self.0.get());
            let len = os::strlen(string);
            let message = std::slice::from_raw_parts(string, len);
            match std::str::from_utf8(message) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => write!(f, "Error code: {}", self.0.get()),
            }
        }
    }
}

impl std::error::Error for Error {}
