// unistd.h

use std::ffi::c_void;
use std::os::raw::c_int;

extern {
    // https://man7.org/linux/man-pages/man2/close.2.html
    pub fn close(fd: c_int) -> c_int;

    // https://man7.org/linux/man-pages/man2/write.2.html
    pub fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
}
