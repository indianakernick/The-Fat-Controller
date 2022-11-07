// fcntl.h
// https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/fcntl.h

use std::ffi::c_int;

pub const O_WRONLY: c_int = 0o00000001;
pub const O_NONBLOCK: c_int = 0o00004000;

extern {
    // https://man7.org/linux/man-pages/man2/open.2.html
    pub fn open(pathname: *const u8, flags: c_int) -> c_int;
}
