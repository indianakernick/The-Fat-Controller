mod kernel;
mod error;

use kernel as lx;

// Some items in the kernel module are actually from the C standard library and
// not the Linux kernel but it doesn't matter.

pub use error::Error;

// https://www.kernel.org/doc/html/latest/input/uinput.html

pub struct Context {

}

impl Context {
    pub fn new() -> Result<Self, Error> {
        Ok(Self{})
    }
}
