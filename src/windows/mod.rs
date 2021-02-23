mod ffi;
mod error;
mod info;
mod keyboard;
mod mouse;

pub use error::Error;

/// The main context used for generating events (Windows).
///
/// The most useful methods are on the [traits](crate::traits).
pub struct Context;

impl Context {
    pub fn new() -> Result<Self, Error> {
        Ok(Self)
    }

    fn send_input(&self, input: &ffi::INPUT) -> Result<(), Error> {
        unsafe {
            if ffi::SendInput(1, input, ffi::SIZEOF_INPUT) == 1 {
                Ok(())
            } else {
                Err(Error::last())
            }
        }
    }
}
