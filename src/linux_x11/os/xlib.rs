// X11/XLib.h
// https://github.com/mirror/libX11/blob/master/include/X11/Xlib.h

use std::os::raw::{c_int, c_uint};

#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct Bool(c_int);
#[repr(transparent)]
pub struct Window(u32);
#[repr(transparent)]
pub struct Display(u8);
#[repr(transparent)]
pub struct Screen(u8);

#[allow(non_upper_case_globals)]
pub const True: Bool = Bool(1);
#[allow(non_upper_case_globals)]
pub const False: Bool = Bool(0);
#[allow(non_upper_case_globals)]
pub const None: Window = Window(0);

#[link(name = "x11")]
extern {
    // https://linux.die.net/man/3/xopendisplay
    pub fn XOpenDisplay(display_name: *const u8) -> *mut Display;

    // https://linux.die.net/man/3/xclosedisplay
    pub fn XCloseDisplay(display: *mut Display) -> c_int;

    // Macro directly accesses struct member
    pub fn XDefaultScreen(display: *mut Display) -> c_int;

    // Macro directly accesses struct member
    // pub fn XDefaultScreenOfDisplay(display: *mut Display) -> *mut Screen;

    // Macro directly accesses struct member
    pub fn XScreenOfDisplay(display: *mut Display, screen_number: c_int) -> *mut Screen;

    // Macro directly accesses struct member
    pub fn XRootWindowOfScreen(screen: *mut Screen) -> Window;

    // Macro directly accesses struct member
    pub fn XWidthOfScreen(screen: *mut Screen) -> c_int;

    // Macro directly accesses struct member
    pub fn XHeightOfScreen(screen: *mut Screen) -> c_int;

    // https://linux.die.net/man/3/xquerypointer
    pub fn XQueryPointer(
        display: *mut Display,
        w: Window,
        root_return: *mut Window,
        child_return: *mut Window,
        root_x_return: *mut c_int,
        root_y_return: *mut c_int,
        win_x_return: *mut c_int,
        win_y_return: *mut c_int,
        mask_return: *mut c_uint,
    ) -> Bool;

    // https://linux.die.net/man/3/xsync
    pub fn XSync(display: *mut Display, discard: Bool) -> c_int;
}
