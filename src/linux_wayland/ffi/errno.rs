// errno.h

use std::ffi::c_int;

extern {
    // https://man7.org/linux/man-pages/man3/errno.3.html
    pub fn __errno_location() -> *mut c_int;
}
