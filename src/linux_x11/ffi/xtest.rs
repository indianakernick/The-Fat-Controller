// X11/extensions/XTest.h
// https://www.x.org/releases/X11R7.7/doc/libXtst/xtestlib.html

use super::{Display, Bool};
use std::os::raw::{c_int, c_uint, c_ulong};

#[allow(non_upper_case_globals)]
pub const CurrentTime: c_ulong = 0;

#[link(name = "Xtst")]
extern {
    pub fn XTestQueryExtension(
        display: *mut Display,
        event_base: *mut c_int,
        error_base: *mut c_int,
        major_version: *mut c_int,
        minor_version: *mut c_int,
    ) -> Bool;

    pub fn XTestFakeKeyEvent(
        display: *mut Display,
        keycode: c_uint,
        is_press: Bool,
        delay: c_ulong
    ) -> c_int;

    pub fn XTestFakeButtonEvent(
        display: *mut Display,
        button: c_uint,
        is_press: Bool,
        delay: c_ulong,
    ) -> c_int;
}
