mod os;
mod error;

use std::ptr;
use std::os::raw::c_int;

pub use error::Error;

pub struct Context {
    display: *mut os::Display,
    screen_number: c_int,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            let display = os::XOpenDisplay(ptr::null());
            if display != ptr::null_mut() {
                let screen_number = os::XDefaultScreen(display);
                Ok(Self {
                    display,
                    screen_number
                })
            } else {
                Err(Error::OpenDisplay)
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            os::XCloseDisplay(self.display);
        }
    }
}
