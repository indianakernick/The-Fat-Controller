// errno.h

use std::os::raw::c_int;

extern {
    // https://man7.org/linux/man-pages/man3/errno.3.html
    pub fn __errno_location() -> *mut c_int;
}
