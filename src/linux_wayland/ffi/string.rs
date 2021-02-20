// string.h

use std::os::raw::c_int;

extern {
    // https://man7.org/linux/man-pages/man3/strerror_l.3.html
    pub fn strerror(errnum: c_int) -> *const u8;

    // https://man7.org/linux/man-pages/man3/strlen.3.html
    pub fn strlen(s: *const u8) -> usize;
}
