mod ffi;
mod error;
mod keyboard;
mod mouse;
mod screen;

use error::PlatformError;
type Error = crate::GenericError<PlatformError>;

/// The main context used for generating events (Windows).
///
/// The most useful methods are on the [`traits`](crate::traits).
pub struct Context;

impl Context {
    pub fn new() -> Result<Self, Error> {
        Ok(Self)
    }

    fn send_input(&self, input: &ffi::INPUT) -> Result<(), Error> {
        self.send_inputs(std::slice::from_ref(input))
    }

    fn send_inputs(&self, inputs: &[ffi::INPUT]) -> Result<(), Error> {
        let len = inputs.len() as ffi::UINT;
        if unsafe { ffi::SendInput(len, inputs.as_ptr(), ffi::SIZEOF_INPUT) } == len {
            Ok(())
        } else {
            Err(Error::Platform(PlatformError::last()))
        }
    }
}
