mod os;
mod error;
mod info;
mod key;
mod mouse;

use std::ptr;
use std::os::raw::c_int;
use crate::linux_common::ScrollAccum;

pub use error::Error;

pub struct Context {
    display: *mut os::Display,
    screen_number: c_int,
    scroll: ScrollAccum,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            let display = os::XOpenDisplay(ptr::null());
            if display == ptr::null_mut() {
                return Err(Error::XOpenDisplay);
            }
            // Passing null pointers for the things we don't need results in a
            // segfault.
            let mut event_base = 0;
            let mut error_base = 0;
            let mut major_version = 0;
            let mut minor_version = 0;
            if os::XTestQueryExtension(
                display,
                &mut event_base,
                &mut error_base,
                &mut major_version,
                &mut minor_version
            ) == os::False {
                return Err(Error::XTestQueryExtension);
            }
            Ok(Self {
                display,
                screen_number: os::XDefaultScreen(display),
                scroll: ScrollAccum::default(),
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
