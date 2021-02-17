mod os;
mod error;
mod info;
mod key;
mod mouse;

pub use error::Error;

/// The main context used for generating events (Windows).
///
/// The most useful methods are on the [`InfoContext`](crate::InfoContext),
/// [`KeyboardContext`](crate::KeyboardContext) and
/// [`MouseContext`](crate::MouseContext) traits.
pub struct Context;

impl Context {
    pub fn new() -> Result<Self, Error> {
        Ok(Self)
    }

    fn send_input(&self, input: &os::INPUT) -> Result<(), Error> {
        unsafe {
            if os::SendInput(1, input, os::SIZEOF_INPUT) == 1 {
                Ok(())
            } else {
                Err(Error::last())
            }
        }
    }
}
