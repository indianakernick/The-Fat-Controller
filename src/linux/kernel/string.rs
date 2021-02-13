// string.h

use std::os::raw::{c_char, c_int};

extern {
    // https://man7.org/linux/man-pages/man3/strerror_l.3.html
    pub fn strerror(errnum: c_int) -> *const c_char;
}
