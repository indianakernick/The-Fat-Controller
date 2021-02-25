use std::ffi::c_void;
use std::os::raw::{c_int, c_uint, c_ushort};
use super::{Bool, Atom, Display, KeyCode, KeySym};

// X11/extensions/XKB.h
#[allow(non_upper_case_globals)]
const XkbNumKbdGroups: usize = 4;

#[repr(C)]
struct XkbSymMapRec {
    kt_index: [u8; XkbNumKbdGroups],
    group_info: u8,
    width: u8,
    offset: c_ushort,
}

type XkbSymMapPtr = *const XkbSymMapRec;

#[repr(C)]
struct XkbClientMapRec {
    size_types: u8,
    num_types: u8,
    types: XkbKeyTypePtr,
    size_syms: c_ushort,
    num_syms: c_ushort,
    syms: *const KeySym,
    key_sym_map: XkbSymMapPtr,
    modmap: *const u8,
}

type XkbClientMapPtr = *const XkbClientMapRec;

#[repr(C)]
pub struct XkbDescRec {
    dpy: *mut Display,
    flags: c_ushort,
    device_spec: c_ushort,
    min_key_code: KeyCode,
    max_key_code: KeyCode,
    ctrls: *const c_void,
    server: *const c_void,
    map: XkbClientMapPtr,
    indicators: *const c_void,
    names: *const c_void,
    compat: *const c_void,
    geom: *const c_void,
}

pub type XkbDescPtr = *const XkbDescRec;

#[allow(non_upper_case_globals)]
const XkbKeyTypesMask: c_uint = 1 << 0;
#[allow(non_upper_case_globals)]
const XkbKeySymsMask: c_uint = 1 << 1;
#[allow(non_upper_case_globals)]
const XkbModifierMapMask: c_uint = 1 << 2;

#[allow(non_upper_case_globals)]
pub const XkbAllClientInfoMask: c_uint = XkbKeyTypesMask |
    XkbKeySymsMask |
    XkbModifierMapMask;

#[allow(non_upper_case_globals)]
pub const XkbUseCoreKbd: c_uint = 0x0100;

// https://www.x.org/releases/X11R7.7/doc/libX11/XKB/xkblib.html
#[repr(C)]
pub struct XkbModsRec {
    pub mask: u8,
    pub real_mods: u8,
    pub vmods: c_ushort,
}

pub type XkbModsPtr = *const XkbModsRec;

#[repr(C)]
pub struct XkbKTMapEntryRec {
    pub active: Bool,
    pub level: u8,
    pub mods: XkbModsRec,
}

pub type XkbKTMapEntryPtr = *const XkbKTMapEntryRec;

#[repr(C)]
pub struct XkbKeyTypeRec {
    pub mods: XkbModsRec,
    pub num_levels: u8,
    pub map_count: u8,
    pub map: XkbKTMapEntryPtr,
    pub preserve: XkbModsPtr,
    pub name: Atom,
    pub level_names: *const Atom,
}

pub type XkbKeyTypePtr = *const XkbKeyTypeRec;

#[allow(non_upper_case_globals)]
pub const ShiftMask: u8 = 1;

#[repr(C)]
pub struct XkbStateRec {
    pub group: u8,
    pub base_group: u8,
    pub latched_group: u8,
    pub locked_group: u8,
    pub mods: u8,
    pub base_mods: u8,
    pub latched_mods: u8,
    pub locked_mods: u8,
    pub compat_state: u8,
    pub grab_mods: u8,
    pub compat_grab_mods: u8,
    pub lookup_mods: u8,
    pub compat_lookup_mods: u8,
    pub ptr_buttons: c_ushort,
}

pub type XkbStatePtr = *mut XkbStateRec;

// These are all macros defined in X11/extensions/XKBstr.h.
// What an unpleasant surprise!

#[allow(non_snake_case)]
fn XkbNumGroups(group_info: u8) -> c_int {
    (group_info & 0xf) as c_int
}

#[allow(non_snake_case)]
unsafe fn XkbCMKeyNumGroups(map: XkbClientMapPtr, keycode: KeyCode) -> c_int {
    XkbNumGroups((*(*map).key_sym_map.add(keycode as usize)).group_info)
}

// https://www.x.org/releases/current/doc/man/man3/XkbKeyNumGroups.3.xhtml
#[allow(non_snake_case)]
pub unsafe fn XkbKeyNumGroups(xkb: XkbDescPtr, keycode: KeyCode) -> c_int {
    XkbCMKeyNumGroups((*xkb).map, keycode)
}

#[allow(non_snake_case)]
unsafe fn XkbCMKeyTypeIndex(map: XkbClientMapPtr, keycode: KeyCode, group: c_int) -> usize {
    (*(*map).key_sym_map.add(keycode as usize)).kt_index[(group & 0x3) as usize] as usize
}

#[allow(non_snake_case)]
unsafe fn XkbCMKeyType(map: XkbClientMapPtr, keycode: KeyCode, group: c_int) -> XkbKeyTypePtr {
    (*map).types.add(XkbCMKeyTypeIndex(map, keycode, group))
}

// https://www.x.org/releases/X11R7.5/doc/man/man3/XkbKeyType.3.html
#[allow(non_snake_case)]
pub unsafe fn XkbKeyKeyType(xkb: XkbDescPtr, keycode: KeyCode, group: c_int) -> XkbKeyTypePtr {
    XkbCMKeyType((*xkb).map, keycode, group)
}

extern {
    // https://www.x.org/releases/X11R7.5/doc/man/man3/XkbGetMap.3.html
    pub fn XkbGetMap(
        display: *mut Display,
        which: c_uint,
        device_spec: c_uint,
    ) -> XkbDescPtr;

    // https://www.x.org/releases/X11R7.5/doc/man/man3/XkbKeycodeToKeysym.3.html
    pub fn XkbKeycodeToKeysym(
        display: *mut Display,
        kc: KeyCode,
        group: c_uint,
        level: c_uint,
    ) -> KeySym;

    // https://www.x.org/releases/current/doc/man/man3/XkbFreeClientMap.3.xhtml
    pub fn XkbFreeClientMap(
        xkb: XkbDescPtr,
        which: c_uint,
        free_all: Bool,
    );

    // https://www.x.org/releases/X11R7.5/doc/man/man3/XkbGetState.3.html
    pub fn XkbGetState(
        display: *mut Display,
        device_spec: c_uint,
        state_return: XkbStatePtr,
    ) -> Bool;

    // https://www.x.org/releases/X11R7.5/doc/man/man3/XkbLockGroup.3.html
    pub fn XkbLockGroup(
        display: *mut Display,
        device_spec: c_uint,
        group: c_uint,
    ) -> Bool;
}
