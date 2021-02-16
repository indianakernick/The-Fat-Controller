mod os;
mod error;
mod info;
mod key;
mod mouse;

pub use error::Error;

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
