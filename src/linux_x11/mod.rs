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
            let null = ptr::null_mut();
            if os::XTestQueryExtension(display, null, null, null, null) == os::False {
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
