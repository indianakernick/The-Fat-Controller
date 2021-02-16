mod os;
mod error;
mod mouse;
mod info;

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
            if display == ptr::null_mut() {
                return Err(Error::OpenDisplay);
            }
            let null = ptr::null_mut();
            if os::XTestQueryExtension(display, null, null, null, null) == os::False {
                return Err(Error::XTestQuery);
            }
            Ok(Self {
                display,
                screen_number: os::XDefaultScreen(display)
            })
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
