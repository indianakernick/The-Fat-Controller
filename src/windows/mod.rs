mod win32;
mod error;
mod key;
mod mouse;
mod info;

use win32 as win;

pub use error::Error;

pub struct Context;

impl Context {
    pub fn new() -> Result<Self, Error> {
        Ok(Context)
    }

    fn send_input(&self, input: &win::INPUT) -> Result<(), Error> {
        unsafe {
            if win::SendInput(1, input, win::SIZEOF_INPUT) == 1 {
                Ok(())
            } else {
                Err(Error::last())
            }
        }
    }
}
