// X11/XLib.h
// https://github.com/mirror/libX11/blob/master/include/X11/Xlib.h

use std::ffi::c_void;
use std::os::raw::{c_int, c_uint, c_ulong};

type XID = c_ulong;

#[derive(Eq, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct Bool(c_int);
#[repr(transparent)]
pub struct Window(XID);
#[repr(transparent)]
pub struct Display(c_void);
#[repr(transparent)]
pub struct Screen(c_void);
#[repr(transparent)]
pub struct Atom(XID);

#[allow(non_upper_case_globals)]
pub const True: Bool = Bool(1);
#[allow(non_upper_case_globals)]
pub const False: Bool = Bool(0);
#[allow(non_upper_case_globals)]
pub const None: Window = Window(0);

pub type KeyCode = u8;

pub type KeySym = XID;

#[allow(non_upper_case_globals)]
pub const NoSymbol: KeySym = 0;

#[repr(C)]
pub struct XModifierKeymap {
    pub max_keypermod: c_int,
    pub modifiermap: *const KeyCode,
}

#[link(name = "X11")]
extern {
    // https://linux.die.net/man/3/xopendisplay
    pub fn XOpenDisplay(display_name: *const u8) -> *mut Display;

    // https://linux.die.net/man/3/xclosedisplay
    pub fn XCloseDisplay(display: *mut Display) -> c_int;

    // Macro directly accesses struct member
    pub fn XDefaultScreen(display: *mut Display) -> c_int;

    // Macro directly accesses struct member
    pub fn XScreenOfDisplay(display: *mut Display, screen_number: c_int) -> *mut Screen;

    // Macro directly accesses struct member
    pub fn XRootWindowOfScreen(screen: *mut Screen) -> Window;

    // Macro directly accesses struct member
    pub fn XWidthOfScreen(screen: *mut Screen) -> c_int;

    // Macro directly accesses struct member
    pub fn XHeightOfScreen(screen: *mut Screen) -> c_int;

    // Macro directly accesses struct member
    pub fn XRootWindow(display: *mut Display, screen_number: c_int) -> Window;

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
    
    // https://linux.die.net/man/3/xwarppointer
    pub fn XWarpPointer(
        display: *mut Display,
        src_w: Window,
        dest_w: Window,
        src_x: c_int,
        src_y: c_int,
        src_width: c_uint,
        src_height: c_uint,
        dest_x: c_int,
        dest_y: c_int,
    ) -> c_int;
    
    // https://linux.die.net/man/3/xsync
    pub fn XSync(display: *mut Display, discard: Bool) -> c_int;
    
    // https://linux.die.net/man/3/xflush
    pub fn XFlush(display: *mut Display) -> c_int;

    // https://tronche.com/gui/x/xlib/input/XDisplayKeycodes.html
    pub fn XDisplayKeycodes(
        display: *mut Display,
        min_keycodes_return: &mut c_int,
        max_keycodes_return: &mut c_int,
    ) -> c_int;

    // https://tronche.com/gui/x/xlib/input/XGetModifierMapping.html
    pub fn XGetModifierMapping(display: *mut Display) -> *const XModifierKeymap;

    pub fn XGetKeyboardMapping(
        display: *mut Display,
        first_keycode: KeyCode,
        keycode_count: c_int,
        keysyms_per_keycode_return: *mut c_int,
    ) -> *const KeySym;

    pub fn XFree(data: *const KeySym) -> c_int;

    // https://tronche.com/gui/x/xlib/utilities/keyboard/XKeysymToString.html
    pub fn XKeysymToString(keysym: KeySym) -> *const u8;

    // https://tronche.com/gui/x/xlib/input/XChangeKeyboardMapping.html
    pub fn XChangeKeyboardMapping(
        display: *mut Display,
        first_keycode: c_int,
        keysyms_per_keycode: c_int,
        keysyms: *const KeySym,
        num_codes: c_int,
    );

    // https://tronche.com/gui/x/xlib/input/XFreeModifiermap.html
    pub fn XFreeModifiermap(modmap: *const XModifierKeymap);
}
